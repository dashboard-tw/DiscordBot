from common import ROOT, arguments, read_json, write_rust

args = arguments('Generate Rust API models without comments or test fixtures.', openapi=True)
root = ROOT
api = read_json(args.openapi)
catalog = read_json(args.catalog)['data']
schemas = dict(api['components']['schemas'])
groups = {topic['id']: [] for topic in catalog['topics']}
groups['common'] = []
owners = {}

def pascal(name):
    return ''.join(word[:1].upper() + word[1:] for word in name.split('_'))

def response_schema(path):
    return api['paths'][path]['get']['responses']['200']['content']['application/json']['schema']

for dataset in catalog['datasets']:
    for suffix in ['latest', 'history', 'sites']:
        path = '/datasets/' + dataset['id'] + '/' + suffix
        if path not in api['paths']:
            continue
        item = response_schema(path)['properties']['data']['items']
        name = item['$ref'].split('/')[-1]
        owners[name] = dataset['topic']

schemas['DatasetDetail'] = response_schema('/datasets/weather_obs')['properties']['data']
for name, schema in list(schemas.items()):
    owners.setdefault(name, 'common')

def rust_name(name):
    return 'ApiError' if name == 'Error' else name

def reference(name):
    return 'super::' + owners[name] + '::' + rust_name(name)

def flatten(schema):
    if 'allOf' not in schema:
        return schema
    result = {'type': 'object', 'properties': {}, 'required': []}
    for part in schema['allOf']:
        if '$ref' in part:
            part = schemas[part['$ref'].split('/')[-1]]
        result['properties'].update(part.get('properties', {}))
        result['required'].extend(part.get('required', []))
    return result

def field_type(field, parent, field_name, module):
    if '$ref' in field:
        return reference(field['$ref'].split('/')[-1])
    kind = field.get('type')
    if isinstance(kind, list):
        kind = next(value for value in kind if value != 'null')
    if kind == 'object' and 'properties' in field:
        name = parent + pascal(field_name)
        if name not in schemas:
            schemas[name] = field
            owners[name] = module
        return name
    if kind == 'array':
        return 'Vec<' + field_type(field['items'], parent, field_name + '_item', module) + '>'
    if kind == 'object':
        return 'std::collections::BTreeMap<String, serde_json::Value>'
    return {'string': 'String', 'number': 'f64', 'integer': 'i64', 'boolean': 'bool'}[kind]

def generate(name, original):
    schema = flatten(original)
    module = owners[name]
    if 'properties' not in schema:
        return f'pub type {rust_name(name)} = std::collections::BTreeMap<String, serde_json::Value>;\n'
    lines = ['#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]',
             'pub struct ' + rust_name(name) + ' {']
    for field_name, field in schema['properties'].items():
        ty = field_type(field, name, field_name, module)
        nullable = 'null' in field.get('type', [])
        if nullable or field_name not in schema.get('required', []):
            ty = 'Option<' + ty + '>'
        ident = 'r#' + field_name if field_name in ['type', 'ref', 'self', 'mod', 'match'] else field_name
        lines.append(f'    pub {ident}: {ty},')
    lines.append('}')
    return '\n'.join(lines) + '\n'

done = set()
while len(done) < len(schemas):
    for name, schema in list(schemas.items()):
        if name not in done:
            groups[owners[name]].append(generate(name, schema))
            done.add(name)

common = '''#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse<T, M = TableMeta> {
    pub data: T,
    #[serde(default)]
    pub meta: M,
}

pub type LatestResponse<T> = ApiResponse<Vec<T>, TableMeta>;
pub type HistoryResponse<T> = ApiResponse<Vec<T>, HistoryMeta>;
pub type SitesResponse<T> = ApiResponse<Vec<T>, TableMeta>;
pub type DatasetListResponse = ApiResponse<Vec<DatasetSummary>, Empty>;
pub type DatasetDetailResponse = ApiResponse<DatasetDetail, Empty>;
'''
groups['common'].append(common)
for dataset in catalog['datasets']:
    topic = dataset['topic']
    prefix = pascal(dataset['id'])
    groups[topic].append(f'pub const {dataset["id"].upper()}_ID: &str = "{dataset["id"]}";\n')
    for suffix in ['latest', 'history', 'sites']:
        path = '/datasets/' + dataset['id'] + '/' + suffix
        if path not in api['paths']:
            continue
        name = response_schema(path)['properties']['data']['items']['$ref'].split('/')[-1]
        alias = prefix + pascal(suffix) + 'Response'
        wrapper = pascal(suffix) + 'Response'
        groups[topic].append(f'pub type {alias} = super::common::{wrapper}<{reference(name)}>;\n')

out = root / 'src/models'
outputs = {out / (module + '.rs'): '\n'.join(structs) for module, structs in groups.items()}
outputs[out / 'mod.rs'] = '\n'.join(f'pub mod {module};' for module in sorted(groups)) + '\n'
write_rust(outputs)
print(f'Generated {len(schemas)} models for {len(catalog["datasets"])} datasets in {len(groups)} modules.')
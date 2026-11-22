import json
from common import ROOT, arguments, read_json, write_rust

args = arguments('Generate user-facing Discord commands and Embed field metadata.')
root = ROOT
catalog = read_json(args.catalog)['data']
outputs = {}
aliases = {'weather_obs':'observation','county_forecast':'forecast','rain_obs':'rain','aqi_obs':'air_quality','uv_obs':'uv','reservoir_obs':'reservoir','bike_availability':'bike','tra_live':'train','ev_live':'charging'}
def literal(value):
    return json.dumps(value, ensure_ascii=False)

metadata = ['pub struct Presentation { pub title: &\'static str, pub fields: &\'static [(&\'static str, &\'static str, &\'static str)], pub site_fields: &\'static [(&\'static str, &\'static str, &\'static str)] }',
            'pub fn presentation(dataset: &str) -> Presentation { match dataset {']
for topic in catalog['topics']:
    datasets = [d for d in catalog['datasets'] if d['topic'] == topic['id']]
    group = topic['id']
    names = [aliases.get(d['id'], d['id']) for d in datasets]
    text = 'use crate::bot::{Context, Error};\nuse crate::http::datasets::Endpoint;\nuse super::datasets::send_dataset;\n'
    text += '#[poise::command(slash_command, subcommands(' + ', '.join(literal(n) for n in names) + '))]\n'
    text += f'pub async fn {group}(_ctx: Context<\'_>) -> Result<(), Error> {{ Ok(()) }}\n'
    for dataset, name in zip(datasets, names):
        identifier = dataset['id']
        text += '#[poise::command(slash_command)]\n'
        text += f'pub async fn {name}(ctx: Context<\'_>,\n'
        if dataset['has_county']:
            text += '#[description = "縣市名稱或五位數代碼，例如臺北市或63000"] county: Option<String>,\n'
        if dataset['has_sites']:
            text += '#[description = "測站或觀測點代碼"] site: Option<String>,\n#[description = "顯示測站清單以查找代碼"] stations: Option<bool>,\n'
        text += '#[description = "顯示頁碼"] page: Option<u32>,\n) -> Result<(), Error> {\n'
        endpoint = 'if stations.unwrap_or(false) { Endpoint::Sites } else { Endpoint::Latest }' if dataset['has_sites'] else 'Endpoint::Latest'
        text += f'send_dataset(ctx, "{identifier}", {endpoint}, {"county" if dataset["has_county"] else "None"}, {"site" if dataset["has_sites"] else "None"}, page).await\n}}\n'
        def fields(key):
            return '&[' + ','.join('('+','.join(literal(f.get(k,'')) for k in ['name','description','unit'])+')' for f in dataset.get(key,[])) + ']'
        metadata.append(f'{literal(identifier)} => Presentation {{ title: {literal(dataset["name"])}, fields: {fields("latest_fields")}, site_fields: {fields("site_fields")} }},')
    text += 'pub fn command() -> poise::Command<crate::bot::Data, Error> {\n'
    text += f'let mut command = {group}();\ncommand.description = Some({literal(topic["name"]+"資訊查詢")}.into());\n'
    text += 'for subcommand in &mut command.subcommands { subcommand.description = Some(match subcommand.name.as_ref() {\n'
    for d,n in zip(datasets,names):
        text += f'{literal(n)} => {literal("查詢"+d["name"])},\n'
    text += '_ => "資訊查詢",\n}.into()); }\ncommand\n}\n'
    outputs[root / f'src/commands/{group}.rs'] = text
metadata.append('_ => Presentation { title: "即時資訊", fields: &[], site_fields: &[] }, } }')
outputs[root / 'src/commands/presentation.rs'] = '\n'.join(metadata)+'\n'
write_rust(outputs)
print(f'Generated commands for {len(catalog["datasets"])} datasets and Embed field metadata.')
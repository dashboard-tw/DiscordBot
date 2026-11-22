#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DatasetSummary {
    pub agency: Option<String>,
    pub description: Option<String>,
    pub has_county: Option<bool>,
    pub has_sites: Option<bool>,
    pub id: Option<String>,
    pub last_observed_at: Option<String>,
    pub license: Option<String>,
    pub name: Option<String>,
    pub source_url: Option<String>,
    pub status: Option<String>,
    pub update_freq_seconds: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DownloadIndex {
    pub dataset: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Vec<super::common::Field>>,
    pub files: Option<Vec<DownloadIndexFilesItem>>,
    pub license: Option<String>,
    pub source: Option<String>,
    pub updated_at: Option<String>,
}

pub type Empty = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiError {
    pub error: ErrorDetail,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Field {
    pub description: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub unit: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HistoryMeta {
    pub dataset: Option<String>,
    pub last_observed_at: Option<String>,
    pub source: Option<super::common::Source>,
    pub status: Option<String>,
    pub from: Option<String>,
    pub next_cursor: Option<String>,
    pub to: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OAuthError {
    pub error: String,
    pub error_description: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Source {
    pub agency: Option<String>,
    pub license: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TableMeta {
    pub dataset: Option<String>,
    pub last_observed_at: Option<String>,
    pub source: Option<super::common::Source>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DatasetDetail {
    pub dataset: Option<super::common::DatasetSummary>,
    pub downloads: Option<String>,
    pub fields: Option<Vec<super::common::Field>>,
    pub has_history: Option<bool>,
    pub history: Option<DatasetDetailHistory>,
    pub latest_fields: Option<Vec<super::common::Field>>,
    pub latest_paged: Option<bool>,
    pub site_fields: Option<Vec<super::common::Field>>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DownloadIndexFilesItem {
    pub bytes: Option<i64>,
    pub file: Option<String>,
    pub generated_at: Option<String>,
    pub month: Option<String>,
    pub rows: Option<i64>,
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DatasetDetailHistory {
    pub max_range_days: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
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

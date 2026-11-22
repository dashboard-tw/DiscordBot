use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use serde::de::DeserializeOwned;
use tokio::sync::Mutex;

use crate::{
    bot::Error,
    config::BASE_URL,
    http::HttpClient,
    models::common::{LatestResponse, SitesResponse},
    token,
};

pub const CACHE_TTL: Duration = Duration::from_secs(600);

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct Query {
    pub county: Option<String>,
    pub site: Option<String>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Endpoint {
    Latest,
    Sites,
}

impl Endpoint {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Latest => "latest",
            Self::Sites => "sites",
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Key {
    dataset: String,
    endpoint: Endpoint,
    query: Query,
}

#[derive(Default)]
struct Entry {
    completed: Option<Instant>,
    result: Option<Result<Arc<serde_json::Value>, String>>,
}

#[derive(Default)]
pub struct DatasetClient {
    entries: Mutex<HashMap<Key, Arc<Mutex<Entry>>>>,
}

pub struct DatasetResponse<T> {
    pub body: T,
    pub cached: bool,
    pub age: Duration,
}

impl DatasetClient {
    pub async fn latest<T: DeserializeOwned>(
        &self,
        dataset: &str,
        query: Query,
    ) -> Result<DatasetResponse<LatestResponse<T>>, Error> {
        self.request(dataset, Endpoint::Latest, query).await
    }

    pub async fn sites<T: DeserializeOwned>(
        &self,
        dataset: &str,
        query: Query,
    ) -> Result<DatasetResponse<SitesResponse<T>>, Error> {
        self.request(dataset, Endpoint::Sites, query).await
    }

    pub async fn request<T: DeserializeOwned>(
        &self,
        dataset: &str,
        endpoint: Endpoint,
        query: Query,
    ) -> Result<DatasetResponse<T>, Error> {
        if dataset.is_empty() || !dataset.bytes().all(|b| b.is_ascii_lowercase() || b == b'_') {
            return Err("Invalid dataset identifier".into());
        }
        let key = Key {
            dataset: dataset.to_owned(),
            endpoint,
            query,
        };
        let entry = {
            let mut entries = self.entries.lock().await;
            if !entries.contains_key(&key) && entries.len() >= 512 {
                entries.retain(|_, entry| {
                    entry
                        .try_lock()
                        .map(|e| {
                            Arc::strong_count(entry) > 1
                                || e.completed.is_some_and(|t| t.elapsed() < CACHE_TTL)
                        })
                        .unwrap_or(true)
                });
                if entries.len() >= 512 {
                    return Err("Dataset query cache is full".into());
                }
            }
            entries.entry(key.clone()).or_default().clone()
        };
        let mut entry = entry.lock().await;
        if let (Some(completed), Some(result)) = (entry.completed, &entry.result) {
            if completed.elapsed() < CACHE_TTL {
                return match result {
                    Ok(body) => Ok(DatasetResponse {
                        body: serde_json::from_value((**body).clone())?,
                        cached: true,
                        age: completed.elapsed(),
                    }),
                    Err(message) => Err(message.clone().into()),
                };
            }
        }
        let result = self
            .fetch(&key)
            .await
            .map(Arc::new)
            .map_err(|e| e.to_string());
        entry.completed = Some(Instant::now());
        entry.result = Some(result.clone());
        match result {
            Ok(body) => Ok(DatasetResponse {
                body: serde_json::from_value((*body).clone())?,
                cached: false,
                age: Duration::ZERO,
            }),
            Err(message) => Err(message.into()),
        }
    }

    async fn fetch(&self, key: &Key) -> Result<serde_json::Value, Error> {
        let token = token::access_token().await?;
        let mut query = Vec::new();
        if let Some(county) = &key.query.county {
            query.push(("county", county.as_str()));
        }
        if let Some(site) = &key.query.site {
            query.push(("site", site.as_str()));
        }
        let mut url = reqwest::Url::parse(&format!(
            "{BASE_URL}/datasets/{}/{}",
            key.dataset,
            key.endpoint.as_str()
        ))?;
        url.query_pairs_mut().extend_pairs(query);
        let mut response = HttpClient::get()
            .client
            .get(url)
            .bearer_auth(token)
            .timeout(Duration::from_secs(30))
            .send()
            .await
            .map_err(|_| "Failed to connect to the dataset service")?;
        if !response.status().is_success() {
            return Err(format!(
                "Dataset service returned HTTP {}; this query can be retried after ten minutes",
                response.status().as_u16()
            )
            .into());
        }
        if response
            .content_length()
            .is_some_and(|size| size > 8 * 1024 * 1024)
        {
            return Err("Dataset response exceeds the size limit; narrow the query scope".into());
        }
        let mut bytes = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| "Dataset service connection interrupted")?
        {
            if bytes.len() + chunk.len() > 8 * 1024 * 1024 {
                return Err(
                    "Dataset response exceeds the size limit; narrow the query scope".into(),
                );
            }
            bytes.extend_from_slice(&chunk);
        }
        let body: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|_| "Invalid dataset response format")?;
        if !body.get("data").is_some_and(|data| data.is_array()) {
            return Err("Invalid dataset response format".into());
        }
        Ok(body)
    }
}

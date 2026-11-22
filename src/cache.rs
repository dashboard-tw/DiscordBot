use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

use crate::cache;

#[derive(Clone, Debug, PartialEq)]
pub enum CacheValue {
    Text(String),
    Int(i64),
    Bool(bool),
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct TokenAnswer {
    pub access_token: String,
    pub expires_in: u64,
}

pub struct Cache {
    pub ready: bool,
    pub token: Option<TokenAnswer>,
    pub data: HashMap<String, CacheValue>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            ready: false,
            token: None,
            data: HashMap::new(),
        }
    }
}

pub type SharedCache = Arc<RwLock<Cache>>;

static CACHE: OnceLock<SharedCache> = OnceLock::new();

pub fn init() {
    CACHE.get_or_init(|| Arc::new(RwLock::new(Cache::new())));
}

pub fn get() -> SharedCache {
    CACHE
        .get_or_init(|| Arc::new(RwLock::new(Cache::new())))
        .clone()
}

pub async fn get_token_answer() -> Result<TokenAnswer, String> {
    let cache = get();
    let data = cache.read().await;

    if let Some(token) = &data.token {
        Ok(token.clone())
    } else {
        Ok(cache::TokenAnswer {
            access_token: "token".to_string(),
            expires_in: 0,
        })
    }
}
pub async fn update_token(token: &str, expires_in: u64) -> Result<(), String> {
    let cache = get();
    let mut data = cache.write().await;

    data.token = Some(TokenAnswer {
        access_token: token.to_string(),
        expires_in,
    });

    Ok(())
}

pub async fn push_value(key: &str, value: &CacheValue) -> Result<(), String> {
    let cache = get();
    let mut data = cache.write().await;

    if data.data.contains_key(key) {
        return Err(format!("Key '{key}' already exists"));
    }

    data.data.insert(key.to_string(), value.clone());

    Ok(())
}

pub async fn get_value(key: &str) -> Option<CacheValue> {
    let cache = get();
    let data = cache.read().await;

    data.data.get(key).cloned()
}

pub async fn set_value(key: &str, value: CacheValue) {
    let cache = get();
    let mut data = cache.write().await;

    data.data.insert(key.to_string(), value);
}

pub async fn remove_value(key: &str) -> Option<CacheValue> {
    let cache = get();
    let mut data = cache.write().await;

    data.data.remove(key)
}

pub async fn clear() {
    let cache = get();
    let mut data = cache.write().await;

    data.data.clear();
}

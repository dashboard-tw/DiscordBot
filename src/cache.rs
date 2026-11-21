
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

#[derive(Clone, Debug, PartialEq)]
pub enum CacheValue {
    Text(String),
    Int(i64),
    Bool(bool),
}

pub struct Cache {
    pub ready: bool,
    pub data: HashMap<String, CacheValue>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            ready: false,
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

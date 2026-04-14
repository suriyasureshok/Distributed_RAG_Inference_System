use std::collections::HashMap;
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait CacheClient {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: String);
}

pub struct MockCacheClient {
    store: Mutex<HashMap<String, String>>,
}

impl MockCacheClient {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl CacheClient for MockCacheClient {
    async fn get(&self, key: &str) -> Option<String> {
        let store = self.store.lock().await;
        store.get(key).cloned()
    }

    async fn set(&self, key: &str, value: String) {
        let mut store = self.store.lock().await;
        store.insert(key.to_string(), value);
    }
}

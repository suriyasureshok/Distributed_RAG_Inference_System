use dashmap::DashMap;
use std::sync::Arc;

use crate::clients::cache_client::CacheClient;

pub struct CacheService {
    l1: DashMap<String, String>,            // in-memory cache
    l2: Arc<dyn CacheClient + Send + Sync>, // Redis client abstraction
}

impl CacheService {
    pub fn new(l2: Arc<dyn CacheClient + Send + Sync>) -> Self {
        Self {
            l1: DashMap::new(),
            l2,
        }
    }

    // GET with read-through logic
    pub async fn get(&self, key: &str) -> Option<String> {
        // L1 check
        if let Some(val) = self.l1.get(key) {
            return Some(val.clone());
        }

        // L2 check
        if let Some(val) = self.l2.get(key).await {
            // populate L1
            self.l1.insert(key.to_string(), val.clone());
            return Some(val);
        }

        None
    }

    // SET with write-back logic
    pub async fn set(&self, key: &str, value: String) {
        // write to L2 first (source of truth)
        self.l2.set(key, value.clone()).await;

        // update L1
        self.l1.insert(key.to_string(), value);
    }
}

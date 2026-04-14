//! # Cache Client
//!
//! Define cache provider abstractions used by the cache service layer.

use std::collections::HashMap;
use tokio::sync::Mutex;

/// Define async cache operations required by the service layer.
#[async_trait::async_trait]
pub trait CacheClient {
    /// Fetch a value by key.
    ///
    /// ## Arguments
    /// - `key`: Cache lookup key.
    ///
    /// ## Returns
    /// Cached value when present, otherwise `None`.
    async fn get(&self, key: &str) -> Option<String>;

    /// Store a value under a key.
    ///
    /// ## Arguments
    /// - `key`: Cache storage key.
    /// - `value`: Cache payload.
    async fn set(&self, key: &str, value: String);
}

/// Provide an in-memory `CacheClient` implementation for local development.
pub struct MockCacheClient {
    store: Mutex<HashMap<String, String>>,
}

impl MockCacheClient {
    /// Create a new empty in-memory cache client.
    ///
    /// ## Returns
    /// A `MockCacheClient` with no preloaded entries.
    pub fn new() -> Self {
        Self {
            store: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait::async_trait]
impl CacheClient for MockCacheClient {
    /// Fetch a value from in-memory storage.
    async fn get(&self, key: &str) -> Option<String> {
        let store = self.store.lock().await;
        store.get(key).cloned()
    }

    /// Store a value in in-memory storage.
    async fn set(&self, key: &str, value: String) {
        let mut store = self.store.lock().await;
        store.insert(key.to_string(), value);
    }
}

//! # Cache Service
//!
//! Implement two-layer caching with read-through and write-back behavior.

use dashmap::DashMap;
use std::sync::Arc;

use crate::clients::cache_client::CacheClient;

/// Provide cache operations across L1 in-memory and L2 external stores.
///
/// ## Fields
/// - `l1`: Fast in-memory cache.
/// - `l2`: Backing cache provider.
pub struct CacheService {
    l1: DashMap<String, String>,            // in-memory cache
    l2: Arc<dyn CacheClient + Send + Sync>, // Redis client abstraction
}

impl CacheService {
    /// Create a new cache service.
    ///
    /// ## Arguments
    /// - `l2`: Backing cache provider implementation.
    ///
    /// ## Returns
    /// A `CacheService` with empty in-memory cache.
    pub fn new(l2: Arc<dyn CacheClient + Send + Sync>) -> Self {
        Self {
            l1: DashMap::new(),
            l2,
        }
    }

    /// Retrieve a value using read-through cache behavior.
    ///
    /// ## Arguments
    /// - `key`: Cache lookup key.
    ///
    /// ## Returns
    /// Cached value when present in L1 or L2, otherwise `None`.
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

    /// Store a value using write-back cache behavior.
    ///
    /// ## Arguments
    /// - `key`: Cache storage key.
    /// - `value`: Cache payload.
    pub async fn set(&self, key: &str, value: String) {
        // write to L2 first (source of truth)
        self.l2.set(key, value.clone()).await;

        // update L1
        self.l1.insert(key.to_string(), value);
    }
}

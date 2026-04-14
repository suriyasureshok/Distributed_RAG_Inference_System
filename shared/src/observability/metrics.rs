//! # Metrics
//!
//! Provide process-local counters for request flow observability.

use std::sync::atomic::{AtomicU64, Ordering};

/// Store global counters used for coarse-grained telemetry.
///
/// ## Fields
/// - `total_requests`: Number of incoming requests.
/// - `cache_hits`: Number of requests served from cache.
/// - `errors`: Number of counted errors.
///
/// ## Thread Safety
/// Uses atomic counters and supports concurrent updates.
pub struct Metrics {
    /// Number of incoming requests.
    pub total_requests: AtomicU64,
    /// Number of requests served from cache.
    pub cache_hits: AtomicU64,
    /// Number of counted errors.
    pub errors: AtomicU64,
}

impl Metrics {
    /// Create a zero-initialized metrics container.
    ///
    /// ## Returns
    /// A `Metrics` instance with all counters set to `0`.
    pub const fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            errors: AtomicU64::new(0),
        }
    }

    /// Increment the total request counter.
    pub fn inc_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment the cache hit counter.
    pub fn inc_cache_hits(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    /// Increment the error counter.
    pub fn inc_errors(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
}

/// Global metrics instance shared by all services in this process.
pub static METRICS: Metrics = Metrics::new();
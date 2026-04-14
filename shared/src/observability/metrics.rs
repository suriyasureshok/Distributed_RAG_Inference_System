use std::sync::atomic::{AtomicU64, Ordering};

pub struct Metrics {
    pub total_requests: AtomicU64,
    pub cache_hits: AtomicU64,
    pub errors: AtomicU64,
}

impl Metrics {
    pub const fn new() -> Self {
        Self {
            total_requests: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            errors: AtomicU64::new(0),
        }
    }

    pub fn inc_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_cache_hits(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn inc_errors(&self) {
        self.errors.fetch_add(1, Ordering::Relaxed);
    }
}

pub static METRICS: Metrics = Metrics::new();
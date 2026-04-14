//! # Coalescing
//!
//! Deduplicate concurrent work for identical request keys.

use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

/// Coalesce concurrent operations by key to avoid duplicate downstream calls.
pub struct Coalescer<T> {
    in_flight: DashMap<String, Arc<InFlight<T>>>,
}

/// Track in-flight task result and waiter notification primitives.
struct InFlight<T> {
    result: Mutex<Option<T>>,
    notify: Notify,
}

impl<T: Clone + Send + Sync + 'static> Coalescer<T> {
    /// Create an empty coalescer.
    ///
    /// ## Returns
    /// A `Coalescer` with no in-flight keys.
    pub fn new() -> Self {
        Self {
            in_flight: DashMap::new(),
        }
    }

    /// Execute task once per key and share result with concurrent waiters.
    ///
    /// ## Type Parameters
    /// - `F`: Task factory closure.
    /// - `Fut`: Future produced by `F`.
    ///
    /// ## Arguments
    /// - `key`: Coalescing key used to group requests.
    /// - `task`: Asynchronous operation to execute once for the key.
    ///
    /// ## Returns
    /// The computed task result.
    pub async fn run<F, Fut>(&self, key: String, task: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = T>,
    {
        let entry = self
            .in_flight
            .entry(key.clone())
            .or_insert_with(|| {
                Arc::new(InFlight {
                    result: Mutex::new(None),
                    notify: Notify::new(),
                })
            })
            .clone();

        // Fast path: check if already computed
        {
            let result_guard = entry.result.lock().await;
            if let Some(result) = &*result_guard {
                return result.clone();
            }
        }

        // Try to become leader
        let result_guard = entry.result.lock().await;

        if result_guard.is_none() {
            // FIRST request executes
            drop(result_guard);

            let result = task().await;

            let mut result_guard = entry.result.lock().await;
            *result_guard = Some(result.clone());

            // wake all waiters
            entry.notify.notify_waiters();

            // cleanup
            self.in_flight.remove(&key);

            result
        } else {
            // WAITERS
            drop(result_guard);

            entry.notify.notified().await;

            let result_guard = entry.result.lock().await;
            result_guard.clone().unwrap()
        }
    }
}

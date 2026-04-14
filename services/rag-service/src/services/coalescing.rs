use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

pub struct Coalescer<T> {
    in_flight: DashMap<String, Arc<InFlight<T>>>,
}

struct InFlight<T> {
    result: Mutex<Option<T>>,
    notify: Notify,
}

impl<T: Clone + Send + Sync + 'static> Coalescer<T> {
    pub fn new() -> Self {
        Self {
            in_flight: DashMap::new(),
        }
    }

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

use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use crate::domain::errors::AppError;

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    Closed,
    Open,
    HalfOpen,
}

pub struct CircuitBreaker {
    state: Mutex<State>,
    failure_count: Mutex<u32>,
    last_failure_time: Mutex<Option<Instant>>,

    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            state: Mutex::new(State::Closed),
            failure_count: Mutex::new(0),
            last_failure_time: Mutex::new(None),
            failure_threshold,
            recovery_timeout,
        }
    }

    pub async fn call<F, Fut, T>(&self, task: F) -> Result<T, AppError>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, AppError>>,
    {
        // Check state
        {
            let mut state = self.state.lock().await;

            if *state == State::Open {
                let last_failure = *self.last_failure_time.lock().await;

                if let Some(last) = last_failure {
                    if last.elapsed() > self.recovery_timeout {
                        *state = State::HalfOpen;
                    } else {
                        return Err(AppError::CircuitOpen);
                    }
                }
            }
        }

        // Execute task
        match task().await {
            Ok(val) => {
                self.on_success().await;
                Ok(val)
            }
            Err(err) => {
                self.on_failure().await;
                Err(err)
            }
        }
    }

    async fn on_success(&self) {
        let mut state = self.state.lock().await;
        let mut failures = self.failure_count.lock().await;

        *failures = 0;
        *state = State::Closed;
    }

    async fn on_failure(&self) {
        let mut failures = self.failure_count.lock().await;
        let mut state = self.state.lock().await;
        let mut last_failure = self.last_failure_time.lock().await;

        *failures += 1;
        *last_failure = Some(Instant::now());

        if *failures >= self.failure_threshold {
            *state = State::Open;
        }
    }
}
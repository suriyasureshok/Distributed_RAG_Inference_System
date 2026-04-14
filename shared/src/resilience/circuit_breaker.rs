//! # Circuit Breaker
//!
//! Provide a simple asynchronous circuit breaker implementation.
//!
//! ## Design
//! The breaker transitions between closed, open, and half-open states based on
//! recent failures and a recovery timeout.

use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use crate::domain::errors::AppError;

/// Represent the current state of the circuit breaker.
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    /// Allow requests and record failures.
    Closed,
    /// Reject requests until recovery timeout elapses.
    Open,
    /// Allow a probe request to determine recovery.
    HalfOpen,
}

/// Guard downstream calls from cascading failures.
///
/// ## Invariants
/// - Transition to `State::Open` after reaching the configured failure threshold.
/// - Transition from `State::Open` to `State::HalfOpen` only after recovery timeout.
/// - Reset to `State::Closed` on successful execution.
pub struct CircuitBreaker {
    state: Mutex<State>,
    failure_count: Mutex<u32>,
    last_failure_time: Mutex<Option<Instant>>,

    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl CircuitBreaker {
    /// Create a new circuit breaker.
    ///
    /// ## Arguments
    /// - `failure_threshold`: Number of failures before opening the circuit.
    /// - `recovery_timeout`: Time to wait before allowing a half-open probe.
    ///
    /// ## Returns
    /// A new `CircuitBreaker` initialized in the closed state.
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            state: Mutex::new(State::Closed),
            failure_count: Mutex::new(0),
            last_failure_time: Mutex::new(None),
            failure_threshold,
            recovery_timeout,
        }
    }

    /// Execute a task while enforcing circuit-breaker state transitions.
    ///
    /// ## Type Parameters
    /// - `F`: Task factory closure.
    /// - `Fut`: Future produced by `F`.
    /// - `T`: Successful result type.
    ///
    /// ## Arguments
    /// - `task`: Asynchronous operation to execute.
    ///
    /// ## Returns
    /// The task result when allowed by the current breaker state.
    ///
    /// ## Errors
    /// Returns `AppError::CircuitOpen` when the circuit is open and recovery timeout
    /// has not elapsed. Returns task errors unchanged when execution fails.
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

    /// Reset failure counters after a successful call.
    async fn on_success(&self) {
        let mut state = self.state.lock().await;
        let mut failures = self.failure_count.lock().await;

        *failures = 0;
        *state = State::Closed;
    }

    /// Record a failure and open the circuit if threshold is reached.
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
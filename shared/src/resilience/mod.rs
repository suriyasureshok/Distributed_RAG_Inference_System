//! # Resilience
//!
//! Provide failure-handling primitives for asynchronous service calls.
//!
//! ## Responsibilities
//! - Retry transient failures.
//! - Enforce operation deadlines.
//! - Prevent cascading failures with a circuit breaker.

/// Provide retry with exponential backoff.
pub mod retry;
/// Provide a circuit breaker implementation.
pub mod circuit_breaker;
/// Provide timeout wrappers for async operations.
pub mod timeout;
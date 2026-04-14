//! # shared
//!
//! Provide common building blocks used by all services in this workspace.
//!
//! ## Overview
//! This crate centralizes cross-cutting concerns:
//! - Domain errors
//! - Resilience primitives
//! - Observability utilities
//! - General helpers
//!
//! ## Features
//! - Uniform `AppError` for service boundaries.
//! - Retry, timeout, and circuit breaker utilities.
//! - Process-local counters and simple logging.
//!
//! ## Example
//! ```rust
//! use shared::utils::semantic_key::semantic_key;
//!
//! let key = semantic_key(" Hello, AI! ");
//! assert_eq!(key, "hello ai");
//! ```

/// Expose domain-level contracts and error types.
pub mod domain;
/// Expose resilience primitives for async workloads.
pub mod resilience;
/// Expose utility helpers used across services.
pub mod utils;
/// Expose logging and metrics helpers.
pub mod observability;
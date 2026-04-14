//! # Observability
//!
//! Provide lightweight logging and metric primitives.
//!
//! ## Responsibilities
//! - Emit timestamped logs.
//! - Track coarse-grained request counters.

/// Provide logging helpers.
pub mod logger;
/// Provide metric counters.
pub mod metrics;
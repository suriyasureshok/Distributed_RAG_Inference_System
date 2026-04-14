//! # Services
//!
//! Provide internal service-layer components used by the application pipeline.

/// Provide two-layer cache orchestration.
pub mod cache_service;
/// Provide in-flight request coalescing.
pub mod coalescing;

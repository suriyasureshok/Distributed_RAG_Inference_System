//! # Service
//!
//! Provide application-level orchestration for LLM generation.
//!
//! ## Responsibilities
//! - Coordinate primary and fallback provider execution.
//! - Apply retry, timeout, and circuit-breaker resilience policies.

/// Provide LLM service orchestration implementation.
pub mod llm_service;

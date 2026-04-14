//! # App Config
//!
//! Define runtime configuration for the RAG service composition root.

use std::time::Duration;

/// Represent application-level runtime settings.
///
/// ## Fields
/// - `embedding_timeout`: Timeout used for embedding calls.
/// - `vector_timeout`: Timeout used for vector search calls.
/// - `llm_timeout`: Timeout used for LLM generation calls.
/// - `max_retries`: Maximum retry attempts for retryable calls.
/// - `circuit_breaker_threshold`: Consecutive failure threshold before opening.
/// - `circuit_breaker_timeout`: Recovery timeout for circuit-breaker probing.
pub struct AppConfig {
    /// Timeout used for embedding calls.
    pub embedding_timeout: Duration,
    /// Timeout used for vector search calls.
    pub vector_timeout: Duration,
    /// Timeout used for LLM generation calls.
    pub llm_timeout: Duration,

    /// Maximum retry attempts for retryable calls.
    pub max_retries: u32,

    /// Consecutive failure threshold before opening the circuit.
    pub circuit_breaker_threshold: u32,
    /// Recovery timeout for circuit-breaker probing.
    pub circuit_breaker_timeout: Duration,
}

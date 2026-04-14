//! # Domain Errors
//!
//! Define shared error types used across services.
//!
//! ## Design
//! Keep failures categorized so resilience policies can decide retry and fallback behavior.

use std::fmt;

/// Represent errors that can occur across the distributed inference workflow.
///
/// ## Variants
/// - `CacheMiss`: Requested entry is not present in cache.
/// - `CacheError`: Cache backend returned an error.
/// - `EmbeddingError`: Embedding generation failed.
/// - `VectorSearchError`: Vector search failed.
/// - `LLMError`: LLM generation failed.
/// - `Timeout`: Operation exceeded configured deadline.
/// - `CircuitOpen`: Circuit breaker rejected the request.
/// - `InternalError`: Internal failure not covered by other categories.
///
/// ## Examples
/// ```rust
/// use shared::domain::errors::AppError;
///
/// let err = AppError::Timeout;
/// assert!(err.is_retryable());
/// ```
#[derive(Debug, Clone)]
pub enum AppError {
    /// Requested entry is not present in cache.
    CacheMiss,
    /// Cache backend returned an error message.
    CacheError(String),

    /// Embedding generation failed.
    EmbeddingError(String),

    /// Vector search failed.
    VectorSearchError(String),

    /// LLM generation failed.
    LLMError(String),

    /// Operation exceeded configured deadline.
    Timeout,
    /// Circuit breaker rejected the request.
    CircuitOpen,

    /// Internal failure not covered by other categories.
    InternalError(String),
}

impl AppError {
    /// Determine whether this error should be retried.
    ///
    /// ## Returns
    /// `true` for transient failures and `false` for terminal failures.
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            AppError::Timeout
                | AppError::EmbeddingError(_)
                | AppError::VectorSearchError(_)
                | AppError::LLMError(_)
        )
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::CacheMiss => write!(f, "Cache miss"),
            AppError::CacheError(e) => write!(f, "Cache error: {}", e),
            AppError::EmbeddingError(e) => write!(f, "Embedding error: {}", e),
            AppError::VectorSearchError(e) => write!(f, "Vector search error: {}", e),
            AppError::LLMError(e) => write!(f, "LLM error: {}", e),
            AppError::Timeout => write!(f, "Timeout occurred"),
            AppError::CircuitOpen => write!(f, "Circuit breaker is open"),
            AppError::InternalError(e) => write!(f, "Internal error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}
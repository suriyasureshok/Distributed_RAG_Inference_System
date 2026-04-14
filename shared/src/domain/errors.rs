use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    // Cache
    CacheMiss,
    CacheError(String),

    // Embedding
    EmbeddingError(String),

    // Vector DB
    VectorSearchError(String),

    // LLM
    LLMError(String),

    // Resilience
    Timeout,
    CircuitOpen,

    // System
    InternalError(String),
}

impl AppError {
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
use std::time::Duration;

pub struct AppConfig {
    pub embedding_timeout: Duration,
    pub vector_timeout: Duration,
    pub llm_timeout: Duration,

    pub max_retries: u32,

    pub circuit_breaker_threshold: u32,
    pub circuit_breaker_timeout: Duration,
}

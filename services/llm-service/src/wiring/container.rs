//! # App Container
//!
//! Construct the concrete dependency graph for llm-service.

use std::sync::Arc;

use crate::service::llm_service::{LLMService, LLMServiceConfig};
use shared::resilience::circuit_breaker::CircuitBreaker;

/// Hold fully-wired application components.
///
/// ## Fields
/// - `llm_service`: Shared LLM service used by transport handlers.
pub struct AppContainer {
    /// Shared LLM service used by transport handlers.
    pub llm_service: Arc<LLMService>,
}

impl AppContainer {
    /// Build the application dependency container.
    ///
    /// ## Returns
    /// An `AppContainer` with configured service dependencies.
    pub fn new() -> Self {
        let circuit_breaker = Arc::new(CircuitBreaker::new(5, std::time::Duration::from_secs(30)));
        let service_config = LLMServiceConfig::from_env();
        let llm_service = Arc::new(LLMService::new(circuit_breaker, service_config));

        Self { llm_service }
    }
}

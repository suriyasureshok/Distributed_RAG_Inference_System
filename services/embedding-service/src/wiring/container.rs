//! # App Container
//!
//! Construct the concrete dependency graph for embedding-service.

use std::sync::Arc;

use crate::service::embedding_service::EmbeddingService;

/// Hold fully-wired application components.
///
/// ## Fields
/// - `embedding_service`: Shared embedding service used by transport handlers.
pub struct AppContainer {
    /// Shared embedding service used by transport handlers.
    pub embedding_service: Arc<EmbeddingService>,
}

impl AppContainer {
    /// Build the application dependency container.
    ///
    /// ## Returns
    /// An `AppContainer` with initialized shared services.
    pub fn new() -> Self {
        let embedding_service = Arc::new(EmbeddingService::new());

        Self { embedding_service }
    }
}
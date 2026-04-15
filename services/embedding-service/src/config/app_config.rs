//! # App Config
//!
//! Define application runtime settings for embedding-service.

/// Represent application-level runtime settings.
///
/// ## Fields
/// - `port`: TCP port used by the HTTP server.
pub struct AppConfig {
    /// TCP port used by the HTTP server.
    pub port: u16,
}

impl AppConfig {
    /// Load application configuration.
    ///
    /// ## Returns
    /// An `AppConfig` with default values for local execution.
    pub fn load() -> Self {
        Self { port: 4001 }
    }
}

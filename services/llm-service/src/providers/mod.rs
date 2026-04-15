//! # Providers
//!
//! Define generation provider implementations used by the service layer.
//!
//! ## Responsibilities
//! - Expose primary provider integration.
//! - Expose fallback provider behavior.

/// Provide fallback generation behavior.
pub mod fallback;
/// Provide primary generation behavior.
pub mod primary;

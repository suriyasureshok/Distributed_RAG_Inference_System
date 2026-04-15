//! # Fallback Provider
//!
//! Implement fallback LLM provider behavior.
//!
//! ## Responsibilities
//! - Return a deterministic response when primary generation fails.

/// Represent the fallback LLM provider.
pub struct FallbackProvider;

impl FallbackProvider {
    /// Generate text using the fallback provider.
    ///
    /// ## Arguments
    /// - `context`: Prompt or context segments for generation.
    ///
    /// ## Returns
    /// Fallback answer text.
    ///
    /// ## Errors
    /// Returns provider errors as a string if fallback generation fails.
    pub async fn generate(&self, context: Vec<String>) -> Result<String, String> {
        Ok(format!("Fallback response for {:?}", context))
    }
}

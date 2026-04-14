//! # Semantic Key
//!
//! Provide normalization logic used for semantic cache keys.

/// Create a normalized semantic key from a query string.
///
/// ## Arguments
/// - `query`: Raw user query text.
///
/// ## Returns
/// A lowercase, trimmed string with non-alphanumeric characters removed,
/// except spaces.
///
/// ## Examples
/// ```rust
/// use shared::utils::semantic_key::semantic_key;
///
/// let key = semantic_key("  Hello, AI!!  ");
/// assert_eq!(key, "hello ai");
/// ```
pub fn semantic_key(query: &str) -> String {
    query
        .trim()
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
}
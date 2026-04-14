pub fn semantic_key(query: &str) -> String {
    query
        .trim()
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != ' ', "")
}
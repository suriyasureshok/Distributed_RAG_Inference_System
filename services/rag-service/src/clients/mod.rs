//! # Clients
//!
//! Provide external dependency adapters used by the RAG pipeline.
//!
//! ## Responsibilities
//! - Expose cache, embedding, vector search, and LLM client abstractions.

/// Provide cache client abstractions and mock implementation.
pub mod cache_client;
/// Provide embedding client implementation.
pub mod embedding_client;
/// Provide LLM client implementation.
pub mod llm_client;
/// Provide vector search client implementation.
pub mod vector_client;

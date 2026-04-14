//! # Application
//!
//! Define application-level orchestration for RAG request handling.
//!
//! ## Responsibilities
//! - Coordinate cache, embedding, retrieval, and generation stages.
//! - Provide a single pipeline API for transport handlers.

/// Provide the core RAG pipeline implementation.
pub mod rag_pipeline;

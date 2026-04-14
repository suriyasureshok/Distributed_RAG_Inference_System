//! # Logger
//!
//! Provide lightweight log emitters for informational and error events.
//!
//! ## Design
//! Logs include a Unix timestamp in milliseconds to simplify cross-service correlation.

use std::time::{SystemTime, UNIX_EPOCH};

/// Emit an informational log line.
///
/// ## Arguments
/// - `message`: Message to print to standard output.
pub fn log_info(message: &str) {
    println!(
        "[INFO][{}] {}",
        timestamp(),
        message
    );
}

/// Emit an error log line.
///
/// ## Arguments
/// - `message`: Message to print to standard error.
pub fn log_error(message: &str) {
    eprintln!(
        "[ERROR][{}] {}",
        timestamp(),
        message
    );
}

/// Return the current Unix timestamp in milliseconds.
fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
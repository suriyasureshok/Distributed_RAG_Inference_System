use std::time::{SystemTime, UNIX_EPOCH};

pub fn log_info(message: &str) {
    println!(
        "[INFO][{}] {}",
        timestamp(),
        message
    );
}

pub fn log_error(message: &str) {
    eprintln!(
        "[ERROR][{}] {}",
        timestamp(),
        message
    );
}

fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
use std::time::Duration;

#[derive(Clone)]
pub struct ClientConfig {
    pub timeout: Duration,
    pub max_retries: u32,
}

use chrono::{DateTime, Utc};

pub struct Cache<T> {
    pub items: Vec<T>,
    pub threads: u64,
    pub max_ram: f64,
    pub expiration: DateTime<Utc>,
    pub requests: u64
}

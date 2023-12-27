use std::time::{Duration, Instant};
use uuid::Uuid;

pub trait JobCallback {
    fn call(&self) {}
}

pub struct Job {
    pub id: Uuid,
    pub interval: Duration,
    pub last_ran_at: Instant,
    pub callback: dyn JobCallback,
}

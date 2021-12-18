use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::utils::has_time_passed;

#[derive(Serialize, Clone)]
pub struct Song {
    data: Option<serde_json::Value>,
    fetched: DateTime<Utc>,
}

impl Song {
    pub fn new(data: Option<serde_json::Value>) -> Song {
        Song {
            data,
            fetched: Utc::now(),
        }
    }

    pub fn is_valid(&self, interval: i64) -> bool {
        !has_time_passed(self.fetched, interval)
    }
}

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;

use crate::INTERVAL_QUERY_SECS;

#[derive(Serialize, Clone)]
pub struct Song {
    data: serde_json::Value,
    fetched: DateTime<Utc>,
}

impl Song {
    pub fn new(data: serde_json::Value) -> Song {
        Song {
            data,
            fetched: Utc::now(),
        }
    }

    pub fn is_valid(&self) -> bool {
        (Utc::now() - Duration::seconds(INTERVAL_QUERY_SECS)) < self.fetched
    }
}

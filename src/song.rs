use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Eq)]
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
}

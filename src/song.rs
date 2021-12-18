use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use std::env;

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

    pub fn is_valid(&self) -> bool {
        (Utc::now()
            - Duration::seconds(
                env::var("INTERVAL_QUERY_SECS")
                    .unwrap_or("10".to_string())
                    .parse::<i64>()
                    .expect("INTERVAL_QUERY_SECS not a i64"),
            ))
            < self.fetched
    }
}

use chrono::{DateTime, Duration, Utc};

pub fn has_time_passed(time: DateTime<Utc>, seconds: i64) -> bool {
    (time + Duration::seconds(seconds)) < Utc::now()
}

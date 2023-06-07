use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_unix_timestamp() -> Duration {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

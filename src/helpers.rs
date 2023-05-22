use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_unix_timestamp() -> Duration {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch;
}
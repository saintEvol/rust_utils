use std::time::{SystemTime, UNIX_EPOCH};

/// 当前微秒数
pub fn now_micros() -> u128 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since.as_micros()
}

/// 当前毫秒数
pub fn now_millis() -> u128 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since.as_millis()
}

///  当前秒数
pub fn now_secs() -> u64 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since.as_secs()
}

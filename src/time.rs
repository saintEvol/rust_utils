use chrono::{Datelike, Local, Timelike};
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

/// 将当前时间秒数转换为字符串
pub fn now_secs_str<T: std::fmt::Display>(suffix: Option<T>) -> String {
    let now = Local::now();
    if let Some(ref s) = suffix {
        let timestamp = format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}{}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
            s
        );
        timestamp
    } else {
        let timestamp = format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}",
            now.year(),
            now.month(),
            now.day(),
            now.hour(),
            now.minute(),
            now.second(),
        );
        timestamp
    }
}

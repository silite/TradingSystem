use chrono::{Duration, NaiveDateTime, TimeZone};

/// UTC时间戳格式化为本地时间
pub fn time_to_local_str(tm: u64, format_opt: Option<&str>) -> String {
    let format = format_opt.unwrap_or("%Y-%m-%d %H:%M:%S%.3f");
    let dt = chrono::FixedOffset::east_opt(8 * 3_600)
        .unwrap()
        .timestamp_millis_opt(tm as i64)
        .unwrap();
    dt.format(format).to_string()
}

/// 本地时间字符串转换为UTC时间戳
pub fn local_str_to_time(tm_str: &str, format_opt: Option<&str>) -> Option<i64> {
    let format = format_opt.unwrap_or("%Y-%m-%d %H:%M:%S%.3f");
    let no_timezone = NaiveDateTime::parse_from_str(tm_str, format);
    if let Ok(tm) = no_timezone {
        Some(tm.and_utc().timestamp_millis() - Duration::hours(8).num_milliseconds())
    } else {
        None
    }
}

/// UTC+0时间字符串转换为UTC时间戳
pub fn utc_str_to_time(tm_str: &str, format_opt: Option<&str>) -> Option<i64> {
    let format = format_opt.unwrap_or("%Y-%m-%d %H:%M:%S%.3f");
    let no_timezone = NaiveDateTime::parse_from_str(tm_str, format);
    if let Ok(tm) = no_timezone {
        Some(tm.and_utc().timestamp_millis())
    } else {
        None
    }
}

/// 本地日期转换为UTC毫秒时间戳, e.g. 20231030
pub fn local_date_to_timestamp_ms(date: i32) -> Option<i64> {
    local_str_to_time(
        format!("{} 00:00:00.000", date).as_str(),
        Some("%Y%m%d %H:%M:%S%.3f"),
    )
}

/// faster than Local::now().timestamp_nanos()
pub fn now_nanos() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

pub fn now_micros() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

pub fn now_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

pub fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u64
}

pub trait TimeFormatter {
    fn millis_str(&self) -> String;
    fn nanos_str(&self) -> String;
}

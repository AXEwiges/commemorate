use chrono::{DateTime, Duration, Local, TimeZone, Utc};
use chrono_tz;
use serde::{Deserialize, Serialize};

use crate::error::{CommemorateError, CommemorateResult};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeInfo {
    pub timestamp: i64,
    pub timezone: String,
}

impl TimeInfo {
    pub fn new(timestamp: i64, timezone: String) -> Self {
        Self {
            timestamp,
            timezone,
        }
    }

    pub fn now(timezone: String) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            timezone,
        }
    }

    pub fn to_local_string(&self) -> CommemorateResult<String> {
        let utc_time = Utc
            .timestamp_opt(self.timestamp, 0)
            .single()
            .ok_or_else(|| CommemorateError::InvalidTimeFormat("Invalid timestamp".to_string()))?;
        let local_time: DateTime<Local> = utc_time.with_timezone(&Local);
        Ok(local_time.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn to_original_timezone_string(&self) -> CommemorateResult<String> {
        let utc_time = Utc
            .timestamp_opt(self.timestamp, 0)
            .single()
            .ok_or_else(|| CommemorateError::InvalidTimeFormat("Invalid timestamp".to_string()))?;
        let tz: chrono_tz::Tz = self
            .timezone
            .parse()
            .map_err(|_| CommemorateError::InvalidTimezone(self.timezone.clone()))?;
        let original_time = utc_time.with_timezone(&tz);
        Ok(original_time.format("%Y-%m-%d %H:%M:%S %Z").to_string())
    }
}

pub fn parse_time(time_str: &str, timezone: &str) -> CommemorateResult<TimeInfo> {
    let tz: chrono_tz::Tz = timezone
        .parse()
        .map_err(|_| CommemorateError::InvalidTimezone(timezone.to_string()))?;
    let naive_time = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d-%H-%M-%S")
        .map_err(|_| CommemorateError::InvalidTimeFormat(time_str.to_string()))?;
    let local_time = tz
        .from_local_datetime(&naive_time)
        .single()
        .ok_or_else(|| CommemorateError::InvalidTimeFormat(time_str.to_string()))?;
    let utc_time = local_time.with_timezone(&Utc);
    Ok(TimeInfo::new(utc_time.timestamp(), timezone.to_string()))
}

pub fn format_duration(duration: Duration) -> String {
    let years = duration.num_days() / 365;
    let days = duration.num_days() % 365;
    let hours = duration.num_hours() % 24;
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;

    let mut parts = Vec::new();
    if years > 0 {
        parts.push(format!(
            "{} year{}",
            years,
            if years > 1 { "s" } else { "" }
        ));
    }
    if days > 0 {
        parts.push(format!("{} day{}", days, if days > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        parts.push(format!(
            "{} hour{}",
            hours,
            if hours > 1 { "s" } else { "" }
        ));
    }
    if minutes > 0 {
        parts.push(format!(
            "{} minute{}",
            minutes,
            if minutes > 1 { "s" } else { "" }
        ));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!(
            "{} second{}",
            seconds,
            if seconds != 1 { "s" } else { "" }
        ));
    }

    parts.join(", ")
}

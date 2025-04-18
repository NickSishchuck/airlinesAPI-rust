use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone, Utc};

/// Format a DateTime to YYYY-MM-DD format
pub fn format_date(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Format a DateTime to HH:MM format
pub fn format_time(date: &DateTime<Utc>) -> String {
    date.format("%H:%M").to_string()
}

/// Format a DateTime to MySQL datetime format
pub fn format_mysql_datetime(date: &DateTime<Utc>) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// Parse a date string in YYYY-MM-DD format
pub fn parse_date(date_str: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
    Ok(DateTime::<Utc>::from_utc(naive_datetime, Utc))
}

/// Calculate duration between two dates in hours and minutes
pub fn calculate_duration(start: &DateTime<Utc>, end: &DateTime<Utc>) -> String {
    let duration = *end - *start;
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    format!("{:02}:{:02}", hours, minutes)
}

/// Parse duration string (like "02:30") to Duration
pub fn parse_duration(duration_str: &str) -> Result<Duration, &'static str> {
    let parts: Vec<&str> = duration_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid duration format. Expected HH:MM");
    }

    let hours: i64 = parts[0].parse().map_err(|_| "Invalid hours value")?;
    let minutes: i64 = parts[1].parse().map_err(|_| "Invalid minutes value")?;

    Ok(Duration::hours(hours) + Duration::minutes(minutes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() {
        let date = Utc.ymd(2023, 1, 15).and_hms(10, 30, 0);
        assert_eq!(format_date(&date), "2023-01-15");
    }

    #[test]
    fn test_format_time() {
        let date = Utc.ymd(2023, 1, 15).and_hms(10, 30, 0);
        assert_eq!(format_time(&date), "10:30");
    }

    #[test]
    fn test_parse_date() {
        let date_str = "2023-01-15";
        let expected = Utc.ymd(2023, 1, 15).and_hms(0, 0, 0);
        assert_eq!(parse_date(date_str).unwrap(), expected);
    }

    #[test]
    fn test_calculate_duration() {
        let start = Utc.ymd(2023, 1, 15).and_hms(10, 30, 0);
        let end = Utc.ymd(2023, 1, 15).and_hms(12, 45, 0);
        assert_eq!(calculate_duration(&start, &end), "02:15");
    }

    #[test]
    fn test_parse_duration() {
        let duration_str = "02:15";
        let expected = Duration::hours(2) + Duration::minutes(15);
        assert_eq!(parse_duration(duration_str).unwrap(), expected);
    }
}
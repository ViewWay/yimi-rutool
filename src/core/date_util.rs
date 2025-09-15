//! Date and time utility functions
//!
//! This module provides comprehensive date and time manipulation utilities,
//! inspired by Hutool's `DateUtil`.

use chrono::{
    DateTime, Datelike, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike,
    Utc, Weekday,
};

/// Date and time utility functions
pub struct DateUtil;

impl DateUtil {
    /// Get current date and time
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{DateTime, Local};
    ///
    /// let now = DateUtil::now();
    /// assert!(now <= Local::now());
    /// ```
    pub fn now() -> DateTime<Local> {
        Local::now()
    }

    /// Get current UTC date and time
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{DateTime, Utc};
    ///
    /// let now_utc = DateUtil::now_utc();
    /// assert!(now_utc <= Utc::now());
    /// ```
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }

    /// Parse date string with specified format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{NaiveDate, Datelike};
    ///
    /// let date = DateUtil::parse_date("2023-12-25", "%Y-%m-%d").unwrap();
    /// assert_eq!(date.year(), 2023);
    /// assert_eq!(date.month(), 12);
    /// assert_eq!(date.day(), 25);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `chrono::ParseError` if the date string cannot be parsed with the given format
    pub fn parse_date(date_str: &str, format: &str) -> Result<NaiveDate, chrono::ParseError> {
        NaiveDate::parse_from_str(date_str, format)
    }

    /// Parse date-time string with specified format
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{NaiveDateTime, Timelike};
    ///
    /// let datetime = DateUtil::parse_datetime("2023-12-25 15:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
    /// assert_eq!(datetime.hour(), 15);
    /// assert_eq!(datetime.minute(), 30);
    /// assert_eq!(datetime.second(), 45);
    /// ```
    pub fn parse_datetime(
        datetime_str: &str,
        format: &str,
    ) -> Result<NaiveDateTime, chrono::ParseError> {
        NaiveDateTime::parse_from_str(datetime_str, format)
    }

    /// Format date to string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let formatted = DateUtil::format_date(date, "%Y-%m-%d");
    /// assert_eq!(formatted, "2023-12-25");
    /// ```
    pub fn format_date(date: NaiveDate, format: &str) -> String {
        date.format(format).to_string()
    }

    /// Format date-time to string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime};
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let time = NaiveTime::from_hms_opt(15, 30, 45).unwrap();
    /// let datetime = NaiveDateTime::new(date, time);
    /// let formatted = DateUtil::format_datetime(datetime, "%Y-%m-%d %H:%M:%S");
    /// assert_eq!(formatted, "2023-12-25 15:30:45");
    /// ```
    pub fn format_datetime(datetime: NaiveDateTime, format: &str) -> String {
        datetime.format(format).to_string()
    }

    /// Format `DateTime` to string | 格式化日期时间为字符串
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{DateTime, Local};
    ///
    /// let now = Local::now();
    /// let formatted = DateUtil::format_datetime_local(now, "%Y-%m-%d %H:%M:%S");
    /// // Should be a valid date-time string
    /// assert!(!formatted.is_empty());
    /// ```
    pub fn format_datetime_local<Tz: TimeZone>(datetime: DateTime<Tz>, format: &str) -> String
    where
        <Tz as TimeZone>::Offset: std::fmt::Display,
    {
        datetime.format(format).to_string()
    }

    /// Get year from date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// assert_eq!(DateUtil::year(date), 2023);
    /// ```
    pub fn year(date: NaiveDate) -> i32 {
        date.year()
    }

    /// Get month from date (1-12)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// assert_eq!(DateUtil::month(date), 12);
    /// ```
    pub fn month(date: NaiveDate) -> u32 {
        date.month()
    }

    /// Get day from date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// assert_eq!(DateUtil::day(date), 25);
    /// ```
    pub fn day(date: NaiveDate) -> u32 {
        date.day()
    }

    /// Get hour from time (0-23)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveTime;
    ///
    /// let time = NaiveTime::from_hms_opt(15, 30, 45).unwrap();
    /// assert_eq!(DateUtil::hour(time), 15);
    /// ```
    pub fn hour(time: NaiveTime) -> u32 {
        time.hour()
    }

    /// Get minute from time (0-59)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveTime;
    ///
    /// let time = NaiveTime::from_hms_opt(15, 30, 45).unwrap();
    /// assert_eq!(DateUtil::minute(time), 30);
    /// ```
    pub fn minute(time: NaiveTime) -> u32 {
        time.minute()
    }

    /// Get second from time (0-59)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveTime;
    ///
    /// let time = NaiveTime::from_hms_opt(15, 30, 45).unwrap();
    /// assert_eq!(DateUtil::second(time), 45);
    /// ```
    pub fn second(time: NaiveTime) -> u32 {
        time.second()
    }

    /// Get day of week
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{NaiveDate, Weekday};
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(); // Monday
    /// assert_eq!(DateUtil::day_of_week(date), Weekday::Mon);
    /// ```
    pub fn day_of_week(date: NaiveDate) -> Weekday {
        date.weekday()
    }

    /// Add days to date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let new_date = DateUtil::add_days(date, 7).unwrap();
    /// assert_eq!(new_date, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    /// ```
    pub fn add_days(date: NaiveDate, days: i64) -> Option<NaiveDate> {
        date.checked_add_signed(Duration::days(days))
    }

    /// Add months to date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let new_date = DateUtil::add_months(date, 1).unwrap();
    /// assert_eq!(new_date, NaiveDate::from_ymd_opt(2024, 1, 25).unwrap());
    /// ```
    pub fn add_months(date: NaiveDate, months: i32) -> Option<NaiveDate> {
        if months >= 0 {
            date.checked_add_months(chrono::Months::new(months as u32))
        } else {
            date.checked_sub_months(chrono::Months::new((-months) as u32))
        }
    }

    /// Add years to date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let new_date = DateUtil::add_years(date, 1).unwrap();
    /// assert_eq!(new_date, NaiveDate::from_ymd_opt(2024, 12, 25).unwrap());
    /// ```
    pub fn add_years(date: NaiveDate, years: i32) -> Option<NaiveDate> {
        let total_months = years * 12;
        if total_months >= 0 {
            date.checked_add_months(chrono::Months::new(total_months as u32))
        } else {
            date.checked_sub_months(chrono::Months::new((-total_months) as u32))
        }
    }

    /// Subtract days from date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let new_date = DateUtil::subtract_days(date, 7).unwrap();
    /// assert_eq!(new_date, NaiveDate::from_ymd_opt(2023, 12, 18).unwrap());
    /// ```
    pub fn subtract_days(date: NaiveDate, days: i64) -> Option<NaiveDate> {
        date.checked_sub_signed(Duration::days(days))
    }

    /// Subtract months from date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let new_date = DateUtil::subtract_months(date, 1).unwrap();
    /// assert_eq!(new_date, NaiveDate::from_ymd_opt(2023, 11, 25).unwrap());
    /// ```
    pub fn subtract_months(date: NaiveDate, months: i32) -> Option<NaiveDate> {
        if months >= 0 {
            date.checked_sub_months(chrono::Months::new(months as u32))
        } else {
            date.checked_add_months(chrono::Months::new((-months) as u32))
        }
    }

    /// Subtract years from date
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let new_date = DateUtil::subtract_years(date, 1).unwrap();
    /// assert_eq!(new_date, NaiveDate::from_ymd_opt(2022, 12, 25).unwrap());
    /// ```
    pub fn subtract_years(date: NaiveDate, years: i32) -> Option<NaiveDate> {
        let total_months = years * 12;
        if total_months >= 0 {
            date.checked_sub_months(chrono::Months::new(total_months as u32))
        } else {
            date.checked_add_months(chrono::Months::new((-total_months) as u32))
        }
    }

    /// Get the difference in days between two dates
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date1 = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let date2 = NaiveDate::from_ymd_opt(2023, 12, 30).unwrap();
    /// assert_eq!(DateUtil::days_between(date1, date2), 5);
    /// assert_eq!(DateUtil::days_between(date2, date1), -5);
    /// ```
    pub fn days_between(date1: NaiveDate, date2: NaiveDate) -> i64 {
        (date2 - date1).num_days()
    }

    /// Check if date is today
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::Local;
    ///
    /// let today = Local::now().date_naive();
    /// assert!(DateUtil::is_today(today));
    ///
    /// let yesterday = DateUtil::subtract_days(today, 1).unwrap();
    /// assert!(!DateUtil::is_today(yesterday));
    /// ```
    pub fn is_today(date: NaiveDate) -> bool {
        date == Local::now().date_naive()
    }

    /// Check if date is in the past
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::Local;
    ///
    /// let yesterday = DateUtil::subtract_days(Local::now().date_naive(), 1).unwrap();
    /// assert!(DateUtil::is_past(yesterday));
    ///
    /// let tomorrow = DateUtil::add_days(Local::now().date_naive(), 1).unwrap();
    /// assert!(!DateUtil::is_past(tomorrow));
    /// ```
    pub fn is_past(date: NaiveDate) -> bool {
        date < Local::now().date_naive()
    }

    /// Check if date is in the future
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::Local;
    ///
    /// let tomorrow = DateUtil::add_days(Local::now().date_naive(), 1).unwrap();
    /// assert!(DateUtil::is_future(tomorrow));
    ///
    /// let yesterday = DateUtil::subtract_days(Local::now().date_naive(), 1).unwrap();
    /// assert!(!DateUtil::is_future(yesterday));
    /// ```
    pub fn is_future(date: NaiveDate) -> bool {
        date > Local::now().date_naive()
    }

    /// Get the first day of the month
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let first_day = DateUtil::first_day_of_month(date);
    /// assert_eq!(first_day, NaiveDate::from_ymd_opt(2023, 12, 1).unwrap());
    /// ```
    pub fn first_day_of_month(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap()
    }

    /// Get the last day of the month
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let last_day = DateUtil::last_day_of_month(date);
    /// assert_eq!(last_day, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    /// ```
    pub fn last_day_of_month(date: NaiveDate) -> NaiveDate {
        let next_month = if date.month() == 12 {
            NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).unwrap()
        };
        next_month.pred_opt().unwrap()
    }

    /// Get the first day of the year
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let first_day = DateUtil::first_day_of_year(date);
    /// assert_eq!(first_day, NaiveDate::from_ymd_opt(2023, 1, 1).unwrap());
    /// ```
    pub fn first_day_of_year(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap()
    }

    /// Get the last day of the year
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let last_day = DateUtil::last_day_of_year(date);
    /// assert_eq!(last_day, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    /// ```
    pub fn last_day_of_year(date: NaiveDate) -> NaiveDate {
        NaiveDate::from_ymd_opt(date.year(), 12, 31).unwrap()
    }

    /// Check if year is a leap year
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    ///
    /// assert!(DateUtil::is_leap_year(2020)); // 2020 is a leap year
    /// assert!(!DateUtil::is_leap_year(2023)); // 2023 is not a leap year
    /// ```
    pub fn is_leap_year(year: i32) -> bool {
        NaiveDate::from_ymd_opt(year, 2, 29).is_some()
    }

    /// Get the number of days in a month
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    ///
    /// assert_eq!(DateUtil::days_in_month(2023, 2), 28); // February 2023
    /// assert_eq!(DateUtil::days_in_month(2020, 2), 29); // February 2020 (leap year)
    /// assert_eq!(DateUtil::days_in_month(2023, 12), 31); // December
    /// ```
    pub fn days_in_month(year: i32, month: u32) -> u32 {
        Self::last_day_of_month(NaiveDate::from_ymd_opt(year, month, 1).unwrap()).day()
    }

    /// Get the number of days in a year
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    ///
    /// assert_eq!(DateUtil::days_in_year(2023), 365); // 2023 is not a leap year
    /// assert_eq!(DateUtil::days_in_year(2020), 366); // 2020 is a leap year
    /// ```
    pub fn days_in_year(year: i32) -> u32 {
        if Self::is_leap_year(year) { 366 } else { 365 }
    }

    /// Parse common date formats automatically
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::NaiveDate;
    ///
    /// let date = DateUtil::parse_auto("2023-12-25").unwrap();
    /// assert_eq!(date, NaiveDate::from_ymd_opt(2023, 12, 25).unwrap());
    ///
    /// let date = DateUtil::parse_auto("12/25/2023").unwrap();
    /// assert_eq!(date, NaiveDate::from_ymd_opt(2023, 12, 25).unwrap());
    /// ```
    pub fn parse_auto(date_str: &str) -> Result<NaiveDate, chrono::ParseError> {
        // Try different common formats
        let formats = [
            "%Y-%m-%d", // 2023-12-25
            "%m/%d/%Y", // 12/25/2023
            "%d/%m/%Y", // 25/12/2023
            "%Y/%m/%d", // 2023/12/25
            "%d-%m-%Y", // 25-12-2023
            "%m-%d-%Y", // 12-25-2023
        ];

        for format in &formats {
            if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
                return Ok(date);
            }
        }

        // If no format matches, return an error for the first format as fallback
        NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
    }

    /// Get Unix timestamp (seconds since 1970-01-01 00:00:00 UTC)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime, TimeZone, Utc};
    ///
    /// let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
    /// let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    /// let datetime = NaiveDateTime::new(date, time);
    /// let timestamp = DateUtil::to_timestamp(datetime);
    /// assert_eq!(timestamp, 1703462400); // 2023-12-25 00:00:00 UTC
    /// ```
    pub fn to_timestamp(datetime: NaiveDateTime) -> i64 {
        datetime.and_utc().timestamp()
    }

    /// Convert Unix timestamp to `NaiveDateTime` | 将Unix时间戳转换为朴素日期时间
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::core::DateUtil;
    /// use chrono::{NaiveDate, NaiveTime, NaiveDateTime, TimeZone, Utc};
    ///
    /// let datetime = DateUtil::from_timestamp(1703462400);
    /// let expected = NaiveDateTime::new(
    ///     NaiveDate::from_ymd_opt(2023, 12, 25).unwrap(),
    ///     NaiveTime::from_hms_opt(0, 0, 0).unwrap()
    /// );
    /// assert_eq!(datetime, expected);
    /// ```
    pub fn from_timestamp(timestamp: i64) -> NaiveDateTime {
        DateTime::from_timestamp(timestamp, 0).unwrap().naive_utc()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_now() {
        let now = DateUtil::now();
        assert!(now <= Local::now());
    }

    #[test]
    fn test_parse_date() {
        let date = DateUtil::parse_date("2023-12-25", "%Y-%m-%d").unwrap();
        assert_eq!(date.year(), 2023);
        assert_eq!(date.month(), 12);
        assert_eq!(date.day(), 25);
    }

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let formatted = DateUtil::format_date(date, "%Y-%m-%d");
        assert_eq!(formatted, "2023-12-25");
    }

    #[test]
    fn test_add_days() {
        let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let new_date = DateUtil::add_days(date, 7).unwrap();
        assert_eq!(new_date, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    }

    #[test]
    fn test_days_between() {
        let date1 = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2023, 12, 30).unwrap();
        assert_eq!(DateUtil::days_between(date1, date2), 5);
        assert_eq!(DateUtil::days_between(date2, date1), -5);
    }

    #[test]
    fn test_first_day_of_month() {
        let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let first_day = DateUtil::first_day_of_month(date);
        assert_eq!(first_day, NaiveDate::from_ymd_opt(2023, 12, 1).unwrap());
    }

    #[test]
    fn test_last_day_of_month() {
        let date = NaiveDate::from_ymd_opt(2023, 12, 25).unwrap();
        let last_day = DateUtil::last_day_of_month(date);
        assert_eq!(last_day, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }

    #[test]
    fn test_is_leap_year() {
        assert!(DateUtil::is_leap_year(2020));
        assert!(!DateUtil::is_leap_year(2023));
        assert!(DateUtil::is_leap_year(2000));
        assert!(!DateUtil::is_leap_year(1900));
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(DateUtil::days_in_month(2023, 2), 28);
        assert_eq!(DateUtil::days_in_month(2020, 2), 29);
        assert_eq!(DateUtil::days_in_month(2023, 12), 31);
    }

    #[test]
    fn test_parse_auto() {
        let date = DateUtil::parse_auto("2023-12-25").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2023, 12, 25).unwrap());

        let date = DateUtil::parse_auto("12/25/2023").unwrap();
        assert_eq!(date, NaiveDate::from_ymd_opt(2023, 12, 25).unwrap());
    }

    #[test]
    fn test_timestamp_conversion() {
        let datetime = DateUtil::from_timestamp(1703462400);
        let timestamp = DateUtil::to_timestamp(datetime);
        assert_eq!(timestamp, 1703462400);
    }
}

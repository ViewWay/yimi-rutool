//! Cron expression parser and validator
//!
//! This module provides functionality to parse and validate cron expressions,
//! supporting standard Unix cron format and extended formats.

use crate::error::{Error, Result};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

#[cfg(feature = "chrono")]
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};

/// Represents a cron expression with parsed fields
#[derive(Debug, Clone, PartialEq)]
pub struct CronExpression {
    /// Seconds (0-59) - optional, not in standard cron
    pub seconds: Option<CronField>,
    /// Minutes (0-59)
    pub minutes: CronField,
    /// Hours (0-23)
    pub hours: CronField,
    /// Day of month (1-31)
    pub day_of_month: CronField,
    /// Month (1-12)
    pub month: CronField,
    /// Day of week (0-7, where both 0 and 7 represent Sunday)
    pub day_of_week: CronField,
    /// Year (optional, 1970-3000)
    pub year: Option<CronField>,
}

/// Represents a single field in a cron expression
#[derive(Debug, Clone, PartialEq)]
pub enum CronField {
    /// All values (*)
    All,
    /// Specific value (e.g., 5)
    Value(u32),
    /// List of values (e.g., 1,3,5)
    List(Vec<u32>),
    /// Range (e.g., 1-5)
    Range(u32, u32),
    /// Step values (e.g., */5, 1-10/2)
    Step(Box<CronField>, u32),
    /// Last day of month (L)
    Last,
    /// Weekday nearest to given day (W)
    Weekday(u32),
    /// Last occurrence of weekday in month (e.g., 5L for last Friday)
    LastWeekday(u32),
    /// Nth occurrence of weekday in month (e.g., 3#2 for second Tuesday)
    NthWeekday(u32, u32),
}

impl CronExpression {
    /// Parse a cron expression from string
    ///
    /// Supports formats:
    /// - Standard: "minute hour day-of-month month day-of-week"
    /// - With seconds: "second minute hour day-of-month month day-of-week"
    /// - With year: "minute hour day-of-month month day-of-week year"
    /// - Full: "second minute hour day-of-month month day-of-week year"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use yimi_rutool::cron::CronExpression;
    ///
    /// // Standard cron: every minute
    /// let expr = CronExpression::parse("* * * * *").unwrap();
    ///
    /// // Every day at 2:30 PM
    /// let expr = CronExpression::parse("30 14 * * *").unwrap();
    ///
    /// // Every Monday at 9:00 AM
    /// let expr = CronExpression::parse("0 9 * * 1").unwrap();
    ///
    /// // With seconds: every 30 seconds
    /// let expr = CronExpression::parse("*/30 * * * * *").unwrap();
    /// ```
    pub fn parse(expression: &str) -> Result<Self> {
        let fields: Vec<&str> = expression.trim().split_whitespace().collect();

        match fields.len() {
            5 => {
                // Standard format: minute hour day month weekday
                Ok(CronExpression {
                    seconds: None,
                    minutes: CronField::parse(fields[0], 0, 59)?,
                    hours: CronField::parse(fields[1], 0, 23)?,
                    day_of_month: CronField::parse(fields[2], 1, 31)?,
                    month: CronField::parse(fields[3], 1, 12)?,
                    day_of_week: CronField::parse(fields[4], 0, 7)?,
                    year: None,
                })
            }
            6 => {
                // Extended format: second minute hour day month weekday
                Ok(CronExpression {
                    seconds: Some(CronField::parse(fields[0], 0, 59)?),
                    minutes: CronField::parse(fields[1], 0, 59)?,
                    hours: CronField::parse(fields[2], 0, 23)?,
                    day_of_month: CronField::parse(fields[3], 1, 31)?,
                    month: CronField::parse(fields[4], 1, 12)?,
                    day_of_week: CronField::parse(fields[5], 0, 7)?,
                    year: None,
                })
            }
            7 => {
                // Full format: second minute hour day month weekday year
                Ok(CronExpression {
                    seconds: Some(CronField::parse(fields[0], 0, 59)?),
                    minutes: CronField::parse(fields[1], 0, 59)?,
                    hours: CronField::parse(fields[2], 0, 23)?,
                    day_of_month: CronField::parse(fields[3], 1, 31)?,
                    month: CronField::parse(fields[4], 1, 12)?,
                    day_of_week: CronField::parse(fields[5], 0, 7)?,
                    year: Some(CronField::parse(fields[6], 1970, 3000)?),
                })
            }
            _ => Err(Error::validation(format!(
                "Invalid cron expression format. Expected 5, 6, or 7 fields, got {}",
                fields.len()
            ))),
        }
    }

    /// Validate the cron expression
    pub fn validate(&self) -> Result<()> {
        // Validate each field
        if let Some(ref seconds) = self.seconds {
            seconds.validate(0, 59, "seconds")?;
        }
        self.minutes.validate(0, 59, "minutes")?;
        self.hours.validate(0, 23, "hours")?;
        self.day_of_month.validate(1, 31, "day_of_month")?;
        self.month.validate(1, 12, "month")?;
        self.day_of_week.validate(0, 7, "day_of_week")?;
        if let Some(ref year) = self.year {
            year.validate(1970, 3000, "year")?;
        }

        // Additional validation logic can be added here
        // For example, checking if day 31 is valid for all months, etc.

        Ok(())
    }

    /// Check if this cron expression matches a given date/time
    #[cfg(feature = "chrono")]
    pub fn matches<Tz: TimeZone>(&self, datetime: &DateTime<Tz>) -> bool {
        // Check seconds
        if let Some(ref seconds) = self.seconds {
            if !seconds.matches(datetime.second()) {
                return false;
            }
        }

        // Check minutes
        if !self.minutes.matches(datetime.minute()) {
            return false;
        }

        // Check hours
        if !self.hours.matches(datetime.hour()) {
            return false;
        }

        // Check day of month
        if !self.day_of_month.matches(datetime.day()) {
            return false;
        }

        // Check month
        if !self.month.matches(datetime.month()) {
            return false;
        }

        // Check day of week (convert to 0-6 range where 0 = Sunday)
        let weekday = datetime.weekday().num_days_from_sunday();
        if !self.day_of_week.matches(weekday) {
            return false;
        }

        // Check year
        if let Some(ref year) = self.year {
            if !year.matches(datetime.year() as u32) {
                return false;
            }
        }

        true
    }

    /// Get the next execution time after the given time
    #[cfg(feature = "chrono")]
    pub fn next_execution(&self, after: &DateTime<Utc>) -> Option<DateTime<Utc>> {
        // This is a simplified implementation
        // A full implementation would need to handle all edge cases
        let mut next = *after + chrono::Duration::minutes(1);

        // Truncate to minute precision if seconds are not specified
        if self.seconds.is_none() {
            next = next.with_second(0).unwrap().with_nanosecond(0).unwrap();
        }

        // Look for the next matching time within a reasonable window
        for _ in 0..366 * 24 * 60 {
            // Max one year
            if self.matches(&next) {
                return Some(next);
            }
            next = next + chrono::Duration::minutes(1);
        }

        None
    }

    /// Get all values that this field matches within its range
    pub fn get_matching_values(&self, field: &CronField, min: u32, max: u32) -> Vec<u32> {
        let mut values = Vec::new();
        for i in min..=max {
            if field.matches(i) {
                values.push(i);
            }
        }
        values
    }
}

impl CronField {
    /// Parse a cron field from string
    pub fn parse(field: &str, min: u32, max: u32) -> Result<Self> {
        let field = field.trim();

        if field == "*" {
            return Ok(CronField::All);
        }

        if field == "L" {
            return Ok(CronField::Last);
        }

        // Handle step values (*/n or range/n)
        if field.contains('/') {
            let parts: Vec<&str> = field.split('/').collect();
            if parts.len() != 2 {
                return Err(Error::validation(format!("Invalid step format: {}", field)));
            }

            let step: u32 = parts[1]
                .parse()
                .map_err(|_| Error::validation(format!("Invalid step value: {}", parts[1])))?;

            if step == 0 {
                return Err(Error::validation("Step value cannot be zero".to_string()));
            }

            let base = if parts[0] == "*" {
                Box::new(CronField::All)
            } else {
                Box::new(CronField::parse(parts[0], min, max)?)
            };

            return Ok(CronField::Step(base, step));
        }

        // Handle ranges (n-m)
        if field.contains('-') {
            let parts: Vec<&str> = field.split('-').collect();
            if parts.len() != 2 {
                return Err(Error::validation(format!(
                    "Invalid range format: {}",
                    field
                )));
            }

            let start: u32 = parts[0]
                .parse()
                .map_err(|_| Error::validation(format!("Invalid range start: {}", parts[0])))?;
            let end: u32 = parts[1]
                .parse()
                .map_err(|_| Error::validation(format!("Invalid range end: {}", parts[1])))?;

            if start > end {
                return Err(Error::validation(format!(
                    "Range start {} is greater than end {}",
                    start, end
                )));
            }

            // Validate range bounds
            if start < min || end > max {
                return Err(Error::validation(format!(
                    "Range [{}, {}] is out of bounds [{}, {}]",
                    start, end, min, max
                )));
            }

            return Ok(CronField::Range(start, end));
        }

        // Handle lists (n,m,o)
        if field.contains(',') {
            let parts: Vec<&str> = field.split(',').collect();
            let mut values = Vec::new();

            for part in parts {
                let value: u32 = part
                    .trim()
                    .parse()
                    .map_err(|_| Error::validation(format!("Invalid list value: {}", part)))?;

                // Validate each value in the list
                if value < min || value > max {
                    return Err(Error::validation(format!(
                        "List value {} is out of range [{}, {}]",
                        value, min, max
                    )));
                }

                values.push(value);
            }

            values.sort();
            values.dedup();
            return Ok(CronField::List(values));
        }

        // Handle special day-of-month expressions
        if field.ends_with('W') {
            let day_str = &field[..field.len() - 1];
            let day: u32 = day_str
                .parse()
                .map_err(|_| Error::validation(format!("Invalid weekday expression: {}", field)))?;
            return Ok(CronField::Weekday(day));
        }

        // Handle last weekday (nL)
        if field.ends_with('L') && field.len() > 1 {
            let weekday_str = &field[..field.len() - 1];
            let weekday: u32 = weekday_str.parse().map_err(|_| {
                Error::validation(format!("Invalid last weekday expression: {}", field))
            })?;
            return Ok(CronField::LastWeekday(weekday));
        }

        // Handle nth weekday (n#m)
        if field.contains('#') {
            let parts: Vec<&str> = field.split('#').collect();
            if parts.len() != 2 {
                return Err(Error::validation(format!(
                    "Invalid nth weekday format: {}",
                    field
                )));
            }

            let weekday: u32 = parts[0].parse().map_err(|_| {
                Error::validation(format!("Invalid weekday in nth expression: {}", parts[0]))
            })?;
            let nth: u32 = parts[1]
                .parse()
                .map_err(|_| Error::validation(format!("Invalid nth value: {}", parts[1])))?;

            return Ok(CronField::NthWeekday(weekday, nth));
        }

        // Handle single value
        let value: u32 = field
            .parse()
            .map_err(|_| Error::validation(format!("Invalid numeric value: {}", field)))?;

        // Validate range immediately
        if value < min || value > max {
            return Err(Error::validation(format!(
                "Value {} is out of range [{}, {}]",
                value, min, max
            )));
        }

        Ok(CronField::Value(value))
    }

    /// Check if this field matches a given value
    pub fn matches(&self, value: u32) -> bool {
        match self {
            CronField::All => true,
            CronField::Value(v) => *v == value,
            CronField::List(values) => values.contains(&value),
            CronField::Range(start, end) => value >= *start && value <= *end,
            CronField::Step(base, step) => {
                if !base.matches(value) {
                    return false;
                }
                match base.as_ref() {
                    CronField::All => value % step == 0,
                    CronField::Range(start, _) => (value - start) % step == 0,
                    _ => value % step == 0,
                }
            }
            CronField::Last => false, // Needs special handling in context
            CronField::Weekday(_) => false, // Needs special handling in context
            CronField::LastWeekday(_) => false, // Needs special handling in context
            CronField::NthWeekday(_, _) => false, // Needs special handling in context
        }
    }

    /// Validate this field against min/max values
    pub fn validate(&self, min: u32, max: u32, field_name: &str) -> Result<()> {
        match self {
            CronField::All => Ok(()),
            CronField::Value(v) => {
                if *v < min || *v > max {
                    Err(Error::validation(format!(
                        "{} value {} is out of range [{}, {}]",
                        field_name, v, min, max
                    )))
                } else {
                    Ok(())
                }
            }
            CronField::List(values) => {
                for &v in values {
                    if v < min || v > max {
                        return Err(Error::validation(format!(
                            "{} value {} is out of range [{}, {}]",
                            field_name, v, min, max
                        )));
                    }
                }
                Ok(())
            }
            CronField::Range(start, end) => {
                if *start < min || *end > max {
                    Err(Error::validation(format!(
                        "{} range [{}, {}] is out of bounds [{}, {}]",
                        field_name, start, end, min, max
                    )))
                } else {
                    Ok(())
                }
            }
            CronField::Step(base, step) => {
                if *step == 0 {
                    return Err(Error::validation(format!(
                        "{} step cannot be zero",
                        field_name
                    )));
                }
                base.validate(min, max, field_name)
            }
            CronField::Last => Ok(()), // Context-dependent validation
            CronField::Weekday(day) => {
                if field_name == "day_of_month" && (*day < 1 || *day > 31) {
                    Err(Error::validation(format!(
                        "Weekday day {} is out of range [1, 31]",
                        day
                    )))
                } else {
                    Ok(())
                }
            }
            CronField::LastWeekday(weekday) => {
                if *weekday > 7 {
                    Err(Error::validation(format!(
                        "Last weekday {} is out of range [0, 7]",
                        weekday
                    )))
                } else {
                    Ok(())
                }
            }
            CronField::NthWeekday(weekday, nth) => {
                if *weekday > 7 {
                    Err(Error::validation(format!(
                        "Nth weekday {} is out of range [0, 7]",
                        weekday
                    )))
                } else if *nth < 1 || *nth > 5 {
                    Err(Error::validation(format!(
                        "Nth occurrence {} is out of range [1, 5]",
                        nth
                    )))
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Get all possible values this field can match
    pub fn get_values(&self, min: u32, max: u32) -> HashSet<u32> {
        let mut values = HashSet::new();

        match self {
            CronField::All => {
                for i in min..=max {
                    values.insert(i);
                }
            }
            CronField::Value(v) => {
                if *v >= min && *v <= max {
                    values.insert(*v);
                }
            }
            CronField::List(list) => {
                for &v in list {
                    if v >= min && v <= max {
                        values.insert(v);
                    }
                }
            }
            CronField::Range(start, end) => {
                let start = (*start).max(min);
                let end = (*end).min(max);
                for i in start..=end {
                    values.insert(i);
                }
            }
            CronField::Step(base, step) => {
                let base_values = base.get_values(min, max);
                for &value in &base_values {
                    if value % step == 0 {
                        values.insert(value);
                    }
                }
            }
            // Special cases would need context-aware implementation
            _ => {}
        }

        values
    }
}

impl FromStr for CronExpression {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        CronExpression::parse(s)
    }
}

impl fmt::Display for CronExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(ref seconds) = self.seconds {
            parts.push(seconds.to_string());
        }

        parts.push(self.minutes.to_string());
        parts.push(self.hours.to_string());
        parts.push(self.day_of_month.to_string());
        parts.push(self.month.to_string());
        parts.push(self.day_of_week.to_string());

        if let Some(ref year) = self.year {
            parts.push(year.to_string());
        }

        write!(f, "{}", parts.join(" "))
    }
}

impl fmt::Display for CronField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CronField::All => write!(f, "*"),
            CronField::Value(v) => write!(f, "{}", v),
            CronField::List(values) => {
                let strs: Vec<String> = values.iter().map(|v| v.to_string()).collect();
                write!(f, "{}", strs.join(","))
            }
            CronField::Range(start, end) => write!(f, "{}-{}", start, end),
            CronField::Step(base, step) => write!(f, "{}/{}", base, step),
            CronField::Last => write!(f, "L"),
            CronField::Weekday(day) => write!(f, "{}W", day),
            CronField::LastWeekday(weekday) => write!(f, "{}L", weekday),
            CronField::NthWeekday(weekday, nth) => write!(f, "{}#{}", weekday, nth),
        }
    }
}

/// Helper for creating common cron expressions
pub struct CronBuilder;

impl CronBuilder {
    /// Create a cron expression that runs every minute
    pub fn every_minute() -> CronExpression {
        CronExpression::parse("* * * * *").unwrap()
    }

    /// Create a cron expression that runs every hour at minute 0
    pub fn every_hour() -> CronExpression {
        CronExpression::parse("0 * * * *").unwrap()
    }

    /// Create a cron expression that runs daily at midnight
    pub fn daily() -> CronExpression {
        CronExpression::parse("0 0 * * *").unwrap()
    }

    /// Create a cron expression that runs daily at specified hour and minute
    pub fn daily_at(hour: u32, minute: u32) -> Result<CronExpression> {
        CronExpression::parse(&format!("{} {} * * *", minute, hour))
    }

    /// Create a cron expression that runs weekly on Sunday at midnight
    pub fn weekly() -> CronExpression {
        CronExpression::parse("0 0 * * 0").unwrap()
    }

    /// Create a cron expression that runs monthly on the 1st at midnight
    pub fn monthly() -> CronExpression {
        CronExpression::parse("0 0 1 * *").unwrap()
    }

    /// Create a cron expression that runs every N minutes
    pub fn every_n_minutes(n: u32) -> Result<CronExpression> {
        if n == 0 || n > 59 {
            return Err(Error::validation(format!("Invalid minute interval: {}", n)));
        }
        CronExpression::parse(&format!("*/{} * * * *", n))
    }

    /// Create a cron expression that runs every N hours
    pub fn every_n_hours(n: u32) -> Result<CronExpression> {
        if n == 0 || n > 23 {
            return Err(Error::validation(format!("Invalid hour interval: {}", n)));
        }
        CronExpression::parse(&format!("0 */{} * * *", n))
    }

    /// Create a cron expression that runs on specific weekdays
    pub fn on_weekdays(weekdays: &[u32]) -> Result<CronExpression> {
        for &day in weekdays {
            if day > 7 {
                return Err(Error::validation(format!("Invalid weekday: {}", day)));
            }
        }
        let weekday_str = weekdays
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join(",");
        CronExpression::parse(&format!("0 0 * * {}", weekday_str))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_standard_cron() {
        let expr = CronExpression::parse("*/5 0 * * 1").unwrap();
        assert!(expr.seconds.is_none());
        assert!(matches!(expr.minutes, CronField::Step(_, 5)));
        assert!(matches!(expr.hours, CronField::Value(0)));
        assert!(matches!(expr.day_of_month, CronField::All));
        assert!(matches!(expr.month, CronField::All));
        assert!(matches!(expr.day_of_week, CronField::Value(1)));
    }

    #[test]
    fn test_parse_extended_cron() {
        let expr = CronExpression::parse("0 */5 0 * * 1").unwrap();
        assert!(expr.seconds.is_some());
        assert!(matches!(expr.seconds, Some(CronField::Value(0))));
        assert!(matches!(expr.minutes, CronField::Step(_, 5)));
    }

    #[test]
    fn test_parse_range() {
        let field = CronField::parse("1-5", 0, 10).unwrap();
        assert!(matches!(field, CronField::Range(1, 5)));
        assert!(field.matches(3));
        assert!(!field.matches(6));
    }

    #[test]
    fn test_parse_list() {
        let field = CronField::parse("1,3,5", 0, 10).unwrap();
        assert!(matches!(field, CronField::List(_)));
        assert!(field.matches(1));
        assert!(field.matches(3));
        assert!(!field.matches(2));
    }

    #[test]
    fn test_parse_step() {
        let field = CronField::parse("*/2", 0, 10).unwrap();
        assert!(matches!(field, CronField::Step(_, 2)));
        assert!(field.matches(0));
        assert!(field.matches(2));
        assert!(!field.matches(1));
    }

    #[test]
    fn test_validation() {
        let expr = CronExpression::parse("0 0 1 1 0").unwrap();
        assert!(expr.validate().is_ok());

        let invalid_expr = CronExpression::parse("60 0 1 1 0");
        assert!(invalid_expr.is_err());
    }

    #[test]
    fn test_cron_builder() {
        let expr = CronBuilder::every_minute();
        assert!(matches!(expr.minutes, CronField::All));

        let expr = CronBuilder::daily_at(9, 30).unwrap();
        assert!(matches!(expr.hours, CronField::Value(9)));
        assert!(matches!(expr.minutes, CronField::Value(30)));

        let expr = CronBuilder::every_n_minutes(15).unwrap();
        assert!(matches!(expr.minutes, CronField::Step(_, 15)));
    }

    #[test]
    fn test_field_get_values() {
        let field = CronField::parse("1,3,5", 0, 10).unwrap();
        let values = field.get_values(0, 10);
        assert_eq!(values.len(), 3);
        assert!(values.contains(&1));
        assert!(values.contains(&3));
        assert!(values.contains(&5));

        let field = CronField::parse("2-6", 0, 10).unwrap();
        let values = field.get_values(0, 10);
        assert_eq!(values.len(), 5);
        for i in 2..=6 {
            assert!(values.contains(&i));
        }
    }

    #[cfg(feature = "chrono")]
    #[test]
    fn test_matches_datetime() {
        use chrono::{TimeZone, Utc};

        let expr = CronExpression::parse("0 9 * * 1").unwrap(); // Every Monday at 9 AM

        // Create a Monday at 9:00 AM
        let monday_9am = Utc.with_ymd_and_hms(2023, 10, 2, 9, 0, 0).unwrap(); // October 2, 2023 was a Monday
        assert!(expr.matches(&monday_9am));

        // Create a Monday at 10:00 AM
        let monday_10am = Utc.with_ymd_and_hms(2023, 10, 2, 10, 0, 0).unwrap();
        assert!(!expr.matches(&monday_10am));

        // Create a Tuesday at 9:00 AM
        let tuesday_9am = Utc.with_ymd_and_hms(2023, 10, 3, 9, 0, 0).unwrap();
        assert!(!expr.matches(&tuesday_9am));
    }

    #[test]
    fn test_display() {
        let expr = CronExpression::parse("*/5 0 1-15 * 1,3,5").unwrap();
        let displayed = expr.to_string();
        assert!(displayed.contains("*/5"));
        assert!(displayed.contains("1-15"));
        assert!(displayed.contains("1,3,5"));
    }

    #[test]
    fn test_error_cases() {
        // Invalid number of fields
        assert!(CronExpression::parse("* *").is_err());
        assert!(CronExpression::parse("* * * * * * * *").is_err());

        // Invalid step
        assert!(CronField::parse("*/0", 0, 59).is_err());

        // Out of range values
        assert!(CronField::parse("60", 0, 59).is_err());
        assert!(CronField::parse("1-60", 0, 59).is_err());

        // Invalid range
        assert!(CronField::parse("5-1", 0, 59).is_err());

        // Invalid builder parameters
        assert!(CronBuilder::every_n_minutes(0).is_err());
        assert!(CronBuilder::every_n_minutes(60).is_err());
        assert!(CronBuilder::daily_at(25, 0).is_err());
    }
}

use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use crate::difference::Difference;
use crate::utils;

const SECONDS_IN_A_DAY: u32 = 3600 * 24;
const SECONDS_IN_A_YEAR: u32 = 3600 * 24 * 365;
const SECONDS_IN_A_LEAP_YEAR: u32 = 3600 * 24 * 366;

pub static JANUARY: u32 = 1;
pub static FEBRUARY: u32 = 2;
pub static MARCH: u32 = 3;
pub static APRIL: u32 = 4;
pub static MAY: u32 = 5;
pub static JUNE: u32 = 6;
pub static JULY: u32 = 7;
pub static AUGUST: u32 = 8;
pub static SEPTEMBER: u32 = 9;
pub static OCTOBER: u32 = 10;
pub static NOVEMBER: u32 = 11;
pub static DECEMBER: u32 = 12;
pub static SUNDAY: u32 = 1;
pub static MONDAY: u32 = 2;
pub static TUESDAY: u32 = 3;
pub static WEDNESDAY: u32 = 4;
pub static THURSDAY: u32 = 5;
pub static FRIDAY: u32 = 6;
pub static SATURDAY: u32 = 7;
pub static DAYS_PER_WEEK: u32 = 7;

#[derive(Debug)]
pub struct DateTime {
    pub year: u32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub weekday: u32,
}

impl DateTime {
    pub fn now() -> Self {
        let current_time = SystemTime::now();
        println!("{:?}", current_time);
        let duration_since = current_time.duration_since(UNIX_EPOCH);
        match duration_since {
            Ok(duration) => println!("{}", duration.as_secs()),
            other=> println!("{:?}", other.unwrap())
        }
        DateTime {
            year: 1999,
            month: 12,
            day: 20,
            hour: 10,
            minute: 59,
            second: 30,
            weekday: 2,
        }
    }

    pub fn utc(year: u32, month: u32, day: u32) -> Self {
        DateTime {
            year: 1999,
            month: 12,
            day: 20,
            hour: 10,
            minute: 59,
            second: 30,
            weekday: 2,
        }
    }

    pub fn parse(utc_string: &str) -> Self {
        let string = utc_string.to_string();
        DateTime {
            year: 1999,
            month: 12,
            day: 20,
            hour: 10,
            minute: 59,
            second: 30,
            weekday: 2,
        }
    }

    pub fn from_milliseconds_since_epoch(milliseconds: u64) -> Self {
        DateTime {
            year: 1999,
            month: 12,
            day: 20,
            hour: 10,
            minute: 59,
            second: 30,
            weekday: 2,
        }
    }

    pub fn to_utc(&self) -> String {
        return String::from("wollup")
    }

    pub fn is_before(&self, other: &DateTime) -> bool {
        return false;
    }

    pub fn is_after(&self, other: DateTime) -> bool {
        return false;
    }

    pub fn is_same_moment_as(&self, other: DateTime) -> bool {
        return false;
    }

    pub fn difference(&self, other: DateTime) -> Difference {
        Difference {
            in_days: 0,
            in_seconds: 35,
            in_milliseconds: 34548
        }
    }

    pub fn add(&self, other: DateTime) -> DateTime {
        let mut sum_of_seconds = self.second + other.second;
        let mut sum_of_minutes = self.minute + other.minute;
        let mut sum_of_hours = self.hour + other.hour;
        let mut sum_of_days = self.day + other.day;
        let mut sum_of_months = self.month + other.month;
        let mut sum_of_years = self.year + other.year;

        if sum_of_seconds > 60 {
            let remainder = sum_of_seconds % 60;
            let quotient = sum_of_seconds / 60;
            sum_of_minutes += quotient * 60;
            sum_of_seconds = remainder;
        }
        if sum_of_minutes > 60 {
            let remainder = sum_of_minutes % 60;
            let quotient = sum_of_minutes / 60;
            sum_of_hours += quotient * 60;
            sum_of_minutes = remainder;
        }
        if sum_of_hours > 24 {
            let remainder = sum_of_minutes % 24;
            let quotient = sum_of_minutes / 24;
            sum_of_days += quotient * 24;
            sum_of_hours = remainder;
        }
        if self.month == FEBRUARY {
            let current_month = FEBRUARY;
            let days_in_current_month = utils::get_days_in_month(current_month, self.year);
            if sum_of_days > days_in_current_month {
                let remainder = sum_of_minutes % 24;
                let quotient = sum_of_minutes / 24;
                sum_of_days += quotient * 24;
                sum_of_hours = remainder;
            }
        }
        // if sum_of_hours > 24 {
        //     let remainder = sum_of_minutes % 24;
        //     let quotient = sum_of_minutes / 24;
        //     sum_of_days += quotient * 24;
        //     sum_of_hours = remainder;
        // }

        DateTime {
            year: sum_of_years,
            month: sum_of_months,
            day: sum_of_days,
            hour: sum_of_hours,
            minute: sum_of_minutes,
            second: sum_of_seconds,
            weekday: 2,
        }
    }

    pub fn subtract(&self, other: DateTime) -> DateTime {
        DateTime {
            year: 1999,
            month: 12,
            day: 20,
            hour: 10,
            minute: 59,
            second: 30,
            weekday: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_dates() {
        let date = DateTime::now();
        println!("{:?}", date);
    }

    #[test]
    fn add_dates_adds_seconds() {
        let first_date = DateTime {
            year: 1990,
            month: 12,
            day: 0,
            hour: 3,
            minute: 45,
            second: 50,
            weekday: 0,
        };
        let second_date = DateTime {
            year: 1990,
            month: 12,
            day: 0,
            hour: 1,
            minute: 45,
            second: 45,
            weekday: 0,
        };
        let result = first_date.add(second_date);
        assert_eq!(result.second, 35);
    }

    #[test]
    fn add_dates_adds_minutes() {
        let first_date = DateTime {
            year: 1990,
            month: 12,
            day: 0,
            hour: 3,
            minute: 45,
            second: 50,
            weekday: 0,
        };
        let second_date = DateTime {
            year: 1990,
            month: 12,
            day: 0,
            hour: 1,
            minute: 45,
            second: 45,
            weekday: 0,
        };
        let result = first_date.add(second_date);
        assert_eq!(result.minute, 30);
    }

    #[test]
    fn add_dates_adds_hours() {
        let first_date = DateTime {
            year: 1990,
            month: 12,
            day: 0,
            hour: 3,
            minute: 45,
            second: 50,
            weekday: 0,
        };
        let second_date = DateTime {
            year: 1990,
            month: 12,
            day: 0,
            hour: 1,
            minute: 45,
            second: 45,
            weekday: 0,
        };
        let result = first_date.add(second_date);
        assert_eq!(result.hour, 6);
    }

    // #[test]
    // #[should_panic]
    // fn parse_panics_when_passed_an_invalid_string() {
    //     DateTime::parse("non-utc string");
    // }
}
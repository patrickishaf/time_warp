use std::time::SystemTime;
use crate::time_warp::difference::Difference;

pub static january: u32 = 1;
pub static february: u32 = 2;
pub static march: u32 = 3;
pub static april: u32 = 4;
pub static may: u32 = 5;
pub static june: u32 = 6;
pub static july: u32 = 7;
pub static august: u32 = 8;
pub static september: u32 = 9;
pub static october: u32 = 10;
pub static november: u32 = 11;
pub static december: u32 = 12;
pub static sunday: u32 = 1;
pub static monday: u32 = 2;
pub static tuesday: u32 = 3;
pub static wednesday: u32 = 4;
pub static thursday: u32 = 5;
pub static friday: u32 = 6;
pub static saturday: u32 = 7;
pub static daysPerWeek: u32 = 7;

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
            in_milliseconds: 34548
        }
    }

    pub fn add(&self, other: DateTime) -> DateTime {
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

    pub fn subract(&self, other: DateTime) -> DateTime {
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
use chrono::prelude::*;
use std::fmt;

const NUM_DAYS_IN_A_WEEK: u32 = 7;
const NUM_WEEKS_IN_A_QUARTER: u32 = 13;
const NUM_QUARTERS_IN_A_YEAR: u32 = 4;
const NUM_DAYS_IN_A_QUARTER: u32 = NUM_DAYS_IN_A_WEEK * NUM_WEEKS_IN_A_QUARTER;
const NUM_OF_DAYS_IN_A_YEAR: u32 = NUM_DAYS_IN_A_QUARTER * NUM_QUARTERS_IN_A_YEAR;

const ORD_FIRST_DAY_OF_YEAR: u32 = 0;
const ORD_LAST_DAY_OF_YEAR: u32 = ORD_FIRST_DAY_OF_YEAR + NUM_OF_DAYS_IN_A_YEAR - 1;
const ORD_NEW_YEARS_DAY: u32 = ORD_LAST_DAY_OF_YEAR + 1;
const ORD_LEAP_DAY: u32 = ORD_NEW_YEARS_DAY + 1;

const NUM_DAYS_FROM_NEW_YEARS_EVE_TO_SPRING_EQUINOX_IN_GREGORIAN_CALENDAR: i32 = 80;

/// Date-clock time.
#[derive(Debug, PartialEq, Eq)]
pub enum Time {
    Regular {
        month: u8,
        week: u8,
        day: u8,
    },
    /// Defined as occurring on the average northward equinox (March 20th).
    NewYears,
    /// Defined as occurring on the day after New Year's day on Leap years.
    LeapDay,
}

impl Time {
    /// Return the current time.
    pub fn now() -> Self {
        Self::from_datelike(Local::now()).unwrap()
    }

    /// Return the corresponding time for the date.
    // we can't use a `TryFrom` impl here because it conflicts with a blanket `TryFrom` impl
    pub fn from_datelike<T: Datelike>(date: T) -> Option<Self> {
        Self::from_num_days_from_ce(date.num_days_from_ce())
    }

    /// Return the corresponding time for the number of days from the Common Era.
    pub fn from_num_days_from_ce(days: i32) -> Option<Self> {
        let adjusted = days - NUM_DAYS_FROM_NEW_YEARS_EVE_TO_SPRING_EQUINOX_IN_GREGORIAN_CALENDAR;
        let ordinal = NaiveDate::from_num_days_from_ce_opt(adjusted)?.ordinal0();
        let value = match ordinal {
            ORD_FIRST_DAY_OF_YEAR..=ORD_LAST_DAY_OF_YEAR => Time::Regular {
                month: pack(ordinal / NUM_DAYS_IN_A_QUARTER),
                week: pack(ordinal / NUM_DAYS_IN_A_WEEK % NUM_WEEKS_IN_A_QUARTER),
                day: pack(ordinal % NUM_DAYS_IN_A_WEEK),
            },
            ORD_NEW_YEARS_DAY => Time::NewYears,
            ORD_LEAP_DAY => Time::LeapDay,
            _ => unreachable!("out of range ordinal: `{ordinal}`"),
        };
        Some(value)
    }
}

fn pack(raw: u32) -> u8 {
    raw.try_into().expect("`raw` should fit into a `u8`")
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Time::*;

        match self {
            Regular { month, week, day } => write!(f, "{month}/{week}/{day}"),
            NewYears => write!(f, "New Year's day"),
            LeapDay => write!(f, "Leap day"),
        }
    }
}

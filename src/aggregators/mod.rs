//! # Aggregators
//!
//! The aggregator functions are a key part of `dta`. Some aggregation functions include:
//!
//! * WIP:
//! ** Range
//! ** Split
//! ** Maximum
//! ** Minimum
//!
//! Note: Not selecting an aggregator when running the `dta` binary will pass the data along
//!
pub mod count;
pub mod max;
pub mod min;
pub mod range;
pub mod split;

use crate::{error, Data, Result};
use chrono::{DateTime, Duration, FixedOffset};
use regex::Regex;
use std::convert::TryFrom;

pub trait Aggregator {
    fn update(&mut self, data: &Data) -> Result<()>;
    fn return_value(&self) -> Result<String>;
}

/// Increment can be used with the rounding trait in chrono todo aggregations by a variable increment.
#[derive(Debug)]
pub struct Increment {
    duration: Duration,
}

impl Increment {
    pub fn from_parts(
        years: i64,
        months: i64,
        days: i64,
        hours: i64,
        minutes: i64,
        seconds: i64,
    ) -> Self {
        Self {
            duration: Duration::weeks(years * 52)
                + Duration::days(months * 30)
                + Duration::days(days)
                + Duration::hours(hours)
                + Duration::minutes(minutes)
                + Duration::seconds(seconds),
        }
    }
    pub fn rounded(&self, dt: DateTime<FixedOffset>) -> Result<DateTime<FixedOffset>> {
        use chrono::DurationRound;
        Ok(dt.duration_round(self.duration)?)
        // Ok(dt)
    }
}

impl TryFrom<String> for Increment {
    type Error = error::Error;
    fn try_from(s: String) -> Result<Increment> {
        let regex_1 = Regex::new(r"^([0-9]{4})\-([0-9]{2})\-([0-9]{2})$")?;
        let regex_2 = Regex::new(r"^([0-9]{2}):([0-9]{2}):([0-9]{2})$")?;
        let regex_3 =
            Regex::new(r"^([0-9]{4})\-([0-9]{2})\-([0-9]{2}) ([0-9]{2}):([0-9]{2}):([0-9]{2})$")?;
        // match on regex 3 then 2 the 1
        if regex_3.is_match(&s) {
            if let Some(matches) = regex_3.captures(&s) {
                if let (
                    Some(years),
                    Some(months),
                    Some(days),
                    Some(hours),
                    Some(minutes),
                    Some(seconds),
                ) = (
                    matches.get(1),
                    matches.get(2),
                    matches.get(3),
                    matches.get(4),
                    matches.get(5),
                    matches.get(6),
                ) {
                    return Ok(Self::from_parts(
                        years.as_str().parse()?,
                        months.as_str().parse()?,
                        days.as_str().parse()?,
                        hours.as_str().parse()?,
                        minutes.as_str().parse()?,
                        seconds.as_str().parse()?,
                    ));
                }
            }
        } else if regex_2.is_match(&s) {
            if let Some(matches) = regex_2.captures(&s) {
                if let (Some(hours), Some(minutes), Some(seconds)) =
                    (matches.get(1), matches.get(2), matches.get(3))
                {
                    return Ok(Self::from_parts(
                        0,
                        0,
                        0,
                        hours.as_str().parse()?,
                        minutes.as_str().parse()?,
                        seconds.as_str().parse()?,
                    ));
                }
            }
        } else if regex_1.is_match(&s) {
            if let Some(matches) = regex_1.captures(&s) {
                if let (Some(years), Some(months), Some(days)) =
                    (matches.get(1), matches.get(2), matches.get(3))
                {
                    return Ok(Self::from_parts(
                        years.as_str().parse()?,
                        months.as_str().parse()?,
                        days.as_str().parse()?,
                        0,
                        0,
                        0,
                    ));
                }
            }
        }
        Err(error::Error {
            reason: format!("Unable to parse {} into increment", s),
            kind: error::ErrorKind::Increment,
        })
    }
}

#[cfg(test)]
mod tests {
    //
    use super::*;
    #[test]
    fn it_works() {
        let i = Increment::try_from("0001-01-01".to_string());
        println!("{:?}", i);
        let i = Increment::try_from("02:06:01".to_string());
        println!("{:?}", i);
        let i = Increment::try_from("0001-01-01 02:06:01".to_string());
        println!("{:?}", i);
    }
}

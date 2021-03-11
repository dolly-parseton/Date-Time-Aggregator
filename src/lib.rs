#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[cfg(feature = "json-parser")]
#[macro_use]
extern crate serde_json;
pub mod aggregators;
mod error;
pub mod input;

use chrono::{DateTime, FixedOffset, NaiveDateTime};
pub use error::Result;

#[derive(Debug, Clone)]
pub struct Data {
    pub timestamp: DateTime<FixedOffset>,
    pub raw: Vec<u8>,
}

impl Default for Data {
    fn default() -> Self {
        Self {
            timestamp: DateTime::from_utc(
                NaiveDateTime::from_timestamp(0, 0),
                chrono::offset::FixedOffset::east(0),
            ),
            raw: Vec::new(),
        }
    }
}

impl Data {
    pub fn new(s: &str, f: Option<&String>, tz: Option<&String>, raw: Vec<u8>) -> Result<Self> {
        // Parse timestamp
        if let Some(timestamp) = datetime_parsing::parse_dt(s, f) {
            return Ok(Data { timestamp, raw });
        }
        if let Some(timestamp) = datetime_parsing::parse_naive_dt(s, f, tz) {
            return Ok(Data { timestamp, raw });
        }
        if let Ok(epoch) = s.parse() {
            if let Some(timestamp) = datetime_parsing::parse_integer(epoch, 0, tz) {
                return Ok(Data { timestamp, raw });
            }
        }
        Err(crate::error::Error {
            reason: format!(
                "{} cannot be parsed. Format string provided {} is not valid.",
                s,
                if f.is_none() {
                    String::new()
                } else {
                    format!("({}) ", f.unwrap())
                }
            ),
            kind: crate::error::ErrorKind::DateTime,
        })
    }
}

mod datetime_parsing {
    //
    lazy_static! {
        static ref time_zone_regex_1: regex::Regex =
            regex::Regex::new(r"(\+|\-)([0-1][0-9]):([0-9]{2})").unwrap();
        static ref time_zone_regex_2: regex::Regex =
            regex::Regex::new(r"(\+|\-)([0-1][0-9])([0-9]{2})").unwrap();
    }
    //
    use super::*;
    use chrono::{DateTime, FixedOffset};
    /// Parse a timestamp field and return a FixedOffset
    pub fn parse_fixed_offset(tz: Option<&String>) -> Result<FixedOffset> {
        if let Some(tz_str) = tz.as_ref() {
            match (
                time_zone_regex_1.captures(tz_str),
                time_zone_regex_2.captures(tz_str),
            ) {
                (Some(captures), None) | (None, Some(captures)) => {
                    let is_east: Option<bool> = captures.get(1).map(|b| b.as_str() == "+");
                    let hours: Option<u16> = match captures.get(2) {
                        Some(h) => Some(h.as_str().parse()?),
                        None => None,
                    };
                    let minutes: Option<u16> = match captures.get(3) {
                        Some(h) => Some(h.as_str().parse()?),
                        None => None,
                    };

                    if is_east.is_some() && hours.is_some() && minutes.is_some() {
                        let timezone = match is_east.unwrap() {
                            true => FixedOffset::east(
                                (3600 * hours.unwrap() + 60 * minutes.unwrap()).into(),
                            ),
                            false => FixedOffset::west(
                                (3600 * hours.unwrap() + 60 * minutes.unwrap()).into(),
                            ),
                        };
                        debug!("Parsed timezone {} from {}", timezone, tz_str);
                        return Ok(timezone);
                    }
                }
                _ => (),
            }
        }
        let err = crate::error::Error {
            reason: match tz {
                Some(tz_str) => format!("Could not convert \"{}\" into a timezone", tz_str),
                None => format!("Could not parse a timezone because no format provided."),
            },
            kind: crate::error::ErrorKind::Timezone,
        };
        error!("Failed to parse timezone: {}", err.reason);
        Err(err)
    }
    /// Parse Integer Timestamps, not currently in use.
    #[allow(dead_code)]
    pub fn parse_integer(i: i64, n: u32, tz: Option<&String>) -> Option<DateTime<FixedOffset>> {
        let timezone = match parse_fixed_offset(tz) {
            Ok(t) => t,
            Err(_) => FixedOffset::east(0),
        };
        match NaiveDateTime::from_timestamp_opt(i, n) {
            Some(dt) => Some(DateTime::from_utc(dt, timezone)),
            None => None,
        }
    }
    // pub fn parse_naive_integer(i: i64, n: i64,  tz: Option<&String>) -> Result<DateTime<FixedOffset>> {
    //     use std::convert::TryInto;
    //     match NaiveDateTime::from_timestamp_opt(i, n.try_into()?) {
    //         Some(dt) => Ok(DateTime::from_utc(dt, parse_fixed_offset(tz)?)),
    //         None => Err(crate::error::Error {
    //             reason: format!(
    //                 "Could not construct a timestamp from {} seconds and {} nanoseconds.",
    //                 i, n
    //             ),
    //             kind: crate::error::ErrorKind::DateTime,
    //         }),
    //     }
    // }
    /// Parse String Timestamp. Returns a NaiveDateTime
    pub fn parse_dt(s: &str, f: Option<&String>) -> Option<DateTime<FixedOffset>> {
        match f {
            Some(fmt) => match DateTime::parse_from_str(s, fmt) {
                Ok(d) => {
                    debug!("Parsed Date with format {:?}: {}", f, d);
                    Some(d)
                }
                Err(_) => None,
            },
            None => {
                if let Ok(d) = DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S %z") {
                    debug!(
                        "Parsed Date with format {:?}: {}",
                        "%Y-%m-%d %H:%M:%S %z", d
                    );
                    Some(d)
                } else if let Ok(d) = DateTime::parse_from_rfc2822(s) {
                    debug!(
                        "Parsed Date with format {:?}: {}",
                        "%a, %d %b %Y %H:%M:%S %Z", d
                    );
                    Some(d)
                } else if let Ok(d) = DateTime::parse_from_rfc3339(s) {
                    debug!(
                        "Parsed Date with format {:?}: {}",
                        "%Y-%m-%dT%H:%M:%S%:z", d
                    );
                    Some(d)
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_naive_dt(
        s: &str,
        f: Option<&String>,
        tz: Option<&String>,
    ) -> Option<DateTime<FixedOffset>> {
        let timezone = match parse_fixed_offset(tz) {
            Ok(t) => t,
            Err(_) => FixedOffset::east(0),
        };
        match f {
            Some(fmt) => match NaiveDateTime::parse_from_str(s, fmt) {
                Ok(d) => {
                    debug!("Parsed Date (Naive) with format {:?}: {}", f, d);
                    Some(DateTime::from_utc(d, timezone))
                }
                Err(_) => None,
            },
            None => {
                if let Ok(d) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                    debug!(
                        "Parsed Date (Naive) with format {:?}: {}",
                        "%Y-%m-%d %H:%M:%S", d
                    );
                    Some(DateTime::from_utc(d, timezone))
                } else if let Ok(d) = NaiveDateTime::parse_from_str(s, "%a, %d %b %Y %H:%M:%S") {
                    debug!(
                        "Parsed Date (Naive) with format {:?}: {}",
                        "%a, %d %b %Y %H:%M:%S", d
                    );
                    Some(DateTime::from_utc(d, timezone))
                } else if let Ok(d) = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S") {
                    debug!(
                        "Parsed Date (Naive) with format {:?}: {}",
                        "%Y-%m-%dT%H:%M:%S", d
                    );
                    Some(DateTime::from_utc(d, timezone))
                } else {
                    None
                }
            }
        }
        // match tz.is_some() {
        //     true => Some(DateTime::from_utc(naive_dt, parse_fixed_offset(tz))),
        //     false => Some(DateTime::from_utc(naive_dt, FixedOffset::east(0))),
        // }
    }
}

#[cfg(test)]
mod tests {
    //
    #[test]
    fn it_works() {
        println!(
            "{:?}",
            chrono::NaiveDateTime::parse_from_str("2020-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap()
        )
    }
}

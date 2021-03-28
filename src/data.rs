use crate::error::{self, Result};
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use std::{collections::HashMap, fmt, fs, path::PathBuf};

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
    pub fn from_dict(
        s: &str,
        raw: Vec<u8>,
        tz: Option<&String>,
        dictionary: &mut parsing::FormatDictionary,
    ) -> Result<Self> {
        Ok(Self {
            timestamp: dictionary.parse_datetime(s, tz)?,
            raw,
        })
    }
    pub fn new(s: &str, f: Option<&String>, tz: Option<&String>, raw: Vec<u8>) -> Result<Self> {
        // Parse timestamp
        if let Some(timestamp) = parsing::parse_dt(s, f) {
            return Ok(Data { timestamp, raw });
        }
        if let Some(timestamp) = parsing::parse_naive_dt(s, f, tz) {
            return Ok(Data { timestamp, raw });
        }
        if let Ok(epoch) = s.parse() {
            if let Some(timestamp) = parsing::parse_integer(epoch, 0, tz) {
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
    pub fn as_string(&self) -> Result<String> {
        use std::str::from_utf8;
        Ok(from_utf8(&self.raw)?.trim_end_matches('\n').to_string())
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.as_string() {
            Ok(s) => write!(f, "{}", s),
            Err(e) => write!(f, "{}", e),
        }
    }
}

pub mod parsing {
    //
    use super::*;
    //
    #[derive(Deserialize, Debug)]
    pub struct FormatDictionary {
        #[serde(flatten)]
        inner: HashMap<String, FormatDictionaryInner>,
        #[serde(skip)]
        priority: Vec<String>,
    }
    #[derive(Deserialize, Debug)]
    struct FormatDictionaryInner {
        fmt: String,
    }

    impl FormatDictionaryInner {
        pub fn get_fmt(&self) -> &str {
            &self.fmt
        }
    }

    impl FormatDictionary {
        pub fn from_file(file: PathBuf) -> Result<Self> {
            //
            let file = fs::File::open(file)?;
            let dict: Self = serde_yaml::from_reader(file)?;
            //
            Ok(dict)
        }
        pub fn parse_datetime(
            &mut self,
            s: &str,
            tz: Option<&String>,
        ) -> Result<DateTime<FixedOffset>> {
            // Accept datetime string and using the formats provided by the dictionary parse out a DateTime<FixedOffset>
            // Read fmts from priority list first
            for name in self.priority.iter() {
                if let Some(fmt) = self.inner.get(name) {
                    if let Ok(dt) = DateTime::parse_from_str(s, fmt.get_fmt()) {
                        //
                        return Ok(dt);
                    } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, fmt.get_fmt()) {
                        return Ok(DateTime::from_utc(
                            dt,
                            match parse_fixed_offset(tz) {
                                Ok(t) => t,
                                Err(_) => FixedOffset::east(0),
                            },
                        ));
                    }
                }
            }
            // If no matches from priority list read from map
            for (name, fmt) in self.inner.iter().map(|(k, v)| (k, v.get_fmt())) {
                if let Ok(dt) = DateTime::parse_from_str(s, fmt) {
                    // Matched. Now update priority list
                    self.priority.insert(0, name.to_string());
                    return Ok(dt);
                } else if let Ok(dt) = NaiveDateTime::parse_from_str(s, fmt) {
                    return Ok(DateTime::from_utc(
                        dt,
                        match parse_fixed_offset(tz) {
                            Ok(t) => t,
                            Err(_) => FixedOffset::east(0),
                        },
                    ));
                }
            }
            Err(error::Error {
                reason: format!("Unable to find format to parse {}", s),
                kind: error::ErrorKind::DateTime,
            })
        }
    }

    //
    lazy_static! {
        static ref TIME_ZONE_REGEX_1: regex::Regex =
            regex::Regex::new(r"(\+|\-)([0-1][0-9]):([0-9]{2})").unwrap();
        static ref TIME_ZONE_REGEX_2: regex::Regex =
            regex::Regex::new(r"(\+|\-)([0-1][0-9])([0-9]{2})").unwrap();
    }
    //
    use chrono::{DateTime, FixedOffset};
    /// Parse a timestamp field and return a FixedOffset
    pub fn parse_fixed_offset(tz: Option<&String>) -> Result<FixedOffset> {
        let timezone_regex_1: regex::Regex = regex::Regex::new(r"(\+|\-)([0-1][0-9]):([0-9]{2})")?;
        let timezone_regex_2: regex::Regex = regex::Regex::new(r"(\+|\-)([0-1][0-9])([0-9]{2})")?;
        if let Some(tz_str) = tz.as_ref() {
            match (
                timezone_regex_1.captures(tz_str),
                timezone_regex_2.captures(tz_str),
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
                    if let (Some(e), Some(h), Some(m)) = (is_east, hours, minutes) {
                        let timezone = match e {
                            true => FixedOffset::east((3600 * h + 60 * m).into()),
                            false => FixedOffset::west((3600 * h + 60 * m).into()),
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
                Some(tz_str) => {
                    format!("Could not convert \"{}\" into a timezone", tz_str)
                }
                None => "Could not parse a timezone because no format provided.".to_string(),
            },
            kind: crate::error::ErrorKind::Timezone,
        };
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
    }
}

#[cfg(test)]
mod tests {
    //
    use super::*;
    #[test]
    fn dict_from_file() {
        let i = parsing::FormatDictionary::from_file(PathBuf::from("./assets/default_formats.yml"));
        println!("{:?}", i);
        assert!(false);
    }
}

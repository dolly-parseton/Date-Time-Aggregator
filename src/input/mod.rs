//! # Input Options
//!
//! Input options currently include CSV and JSON, there are also options for auto detecting the format that allows you to use glob path matching or piping to take in data.
//!
//! There are two features for input options `csv-input` and `json-input`.
//! If no file input is select data is read line by line from standard input

// Add in CSV and JSON inputs if feature selected
#[cfg(feature = "csv-parser")]
pub mod csv;
#[cfg(feature = "file-input")]
pub mod file;
#[cfg(feature = "json-parser")]
pub mod json;
#[cfg(feature = "stdin-input")]
pub mod stdin;

// Uses
use crate::{Data, Result};
use chrono::{DateTime, FixedOffset};

/// Source Trait can be used to read in raw bytes, the struct the trait is implimented on holds the position.
pub trait Source {
    /// Read an entry from source location
    fn read_data(&self) -> Result<Vec<u8>>;
}

/// Parser Trait can be implimented to read in raw data from a [`Source`](crate::input::Source) using an option provided
pub trait Parser {
    /// Read an entry from source location.
    fn parse_data(
        &self,
        raw: Vec<u8>,
        _field: &str,
        fmt: Option<&String>,
        tz: Option<&String>,
    ) -> Result<Data>;
}

pub mod simple {
    use crate::{
        error::{Error, ErrorKind},
        input::Parser,
        Data, Result,
    };
    use chrono::{DateTime, FixedOffset};

    pub struct SimpleParser;

    impl Default for SimpleParser {
        fn default() -> Self {
            Self
        }
    }
    impl Parser for SimpleParser {
        fn parse_data(
            &self,
            raw: Vec<u8>,
            _field: &str,
            fmt: Option<&String>,
            tz: Option<&String>,
        ) -> Result<Data> {
            // Parse raw data back into a string
            use std::str;
            match str::from_utf8(&raw.clone()) {
                Ok(t) => {
                    let data = Data::new(t, fmt, tz, raw)?;
                    debug!("Parsed data from raw bytes: {:?}", data);
                    Ok(data)
                }
                Err(e) => {
                    let err = Error {
                        reason: format!("Timestamp could not be parsed: {}", e),
                        kind: ErrorKind::Parser,
                    };
                    error!("Error occured during parsing: {:?}", err);
                    Err(err)
                }
            }
        }
    }
}

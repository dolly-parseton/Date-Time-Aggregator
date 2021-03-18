//! # Input Options
//!
//! Input options currently include CSV and JSON, there are also options for auto detecting the format that allows you to use glob path matching or piping to take in data.
//!
//! There are two features for input options `csv-input` and `json-input`.
//! If no file input is select data is read line by line from standard input

// Add in CSV and JSON inputs if feature selected
pub mod csv;
pub mod file;
pub mod json;
pub mod stdin;

// Uses
use crate::{Data, Result};

/// Source Trait can be used to read in raw bytes, the struct the trait is implimented on holds the position.
pub trait Source {
    /// Read an entry from source location
    fn read_data(&mut self) -> Result<Vec<u8>>;
}

/// Parser Trait can be implimented to read in raw data from a [`Source`](crate::input::Source) using an option provided
pub trait Parser {
    /// Read an entry from source location.
    fn parse_data(
        &self,
        raw: Vec<u8>,
        fmt: Option<&String>,
        tz: Option<&String>,
        dict: Option<&mut crate::FormatDictionary>,
    ) -> Result<Data>;
}

pub mod simple {
    use crate::{
        error::{Error, ErrorKind},
        input::Parser,
        Data, Result,
    };

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
            fmt: Option<&String>,
            tz: Option<&String>,
            dict: Option<&mut crate::FormatDictionary>,
        ) -> Result<Data> {
            // Parse raw data back into a string
            use std::str;
            match str::from_utf8(&raw) {
                Ok(t) => {
                    let data = match dict {
                        Some(d) => Data::from_dict(&t, raw.clone(), tz, d)?,
                        None => Data::new(&t, fmt, tz, raw.clone())?,
                    };
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

//! # Json Parser
//!
//! The Json parser is used to read in data from a Json source and parse out a date time field to be used in aggregation.
//!
use crate::{
    error::{Error, ErrorKind},
    input::Parser,
    Data, Result,
};

pub struct JsonParser {
    field: String,
}

impl JsonParser {
    pub fn new(field: String) -> Self {
        Self { field }
    }
}

impl Parser for JsonParser {
    fn parse_data(
        &self,
        raw: Vec<u8>,
        // field: Option<&String>,
        fmt: Option<&String>,
        tz: Option<&String>,
    ) -> Result<Data> {
        // Parse raw data back into a string
        use std::str;
        let data = match str::from_utf8(&raw[..]) {
            Ok(d) => d,
            Err(e) => {
                let err = Error {
                    reason: format!("Data coverted: {}", e),
                    kind: ErrorKind::Parser,
                };
                error!("Error occured during parsing: {:?}", err);
                return Err(err);
            }
        };

        match serde_json::from_str::<serde_json::Value>(data) {
            Ok(v) => {
                if let Some(ts_value) = v.get(&self.field) {
                    println!("{:?}", ts_value);
                    if let Some(ts_str) = ts_value.as_str() {
                        let data = Data::new(ts_str, fmt, tz, raw)?;
                        debug!("Parsed data from raw bytes: {:?}", data);
                        return Ok(data);
                    }
                }
                let err = Error {
                    reason: format!("Could no find a field named {}", self.field),
                    kind: ErrorKind::Parser,
                };
                error!("Error occured during parsing: {:?}", err);
                return Err(err);
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

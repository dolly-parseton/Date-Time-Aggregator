//! # Json Parser
//!
//! The Json parser is used to read in data from a Json source and parse out a date time field to be used in aggregation.
//!
//! Todo: Add nested field support (recursive function that runs when there is a '.' in the field string provided).
use crate::{
    error::{Error, ErrorKind},
    input::Parser,
    Data, Result,
};

pub struct JsonParser {
    field: String,
}

impl JsonParser {
    pub fn new(field: &str) -> Self {
        Self {
            field: field.to_string(),
        }
    }
}

impl Parser for JsonParser {
    fn parse_data(
        &self,
        raw: Vec<u8>,
        // field: Option<&String>,
        fmt: Option<&String>,
        tz: Option<&String>,
        dict: Option<&mut crate::FormatDictionary>,
        transform: Option<&String>,
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
            Ok(mut v) => {
                if let Some(ts_value) = v.get(&self.field) {
                    if let Some(ts_str) = ts_value.as_str() {
                        let data = match dict {
                            Some(mut d) => Data::from_dict(&ts_str, raw, tz, &mut d)?,
                            None => Data::new(&ts_str, fmt, tz, raw)?,
                        };
                        // If transform exists modify the value enum and
                        if let (Some(t), Some(v_mut)) = (transform, v.get_mut(&self.field)) {
                            let dt = data.timestamp.format(t).to_string();
                            *v_mut = serde_json::Value::String(dt);
                        }
                        debug!("Parsed data from raw bytes: {:?}", data);
                        return Ok(data);
                    }
                }
                let err = Error {
                    reason: format!("Could no find a field named {}", self.field),
                    kind: ErrorKind::Parser,
                };
                error!("Error occured during parsing: {:?}", err);
                Err(err)
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

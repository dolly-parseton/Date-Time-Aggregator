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

        let value = gjson::get(&data, &self.field);
        let ts_str = value.str();
        let mut data = match dict {
            Some(mut d) => Data::from_dict(&ts_str, data.as_bytes().to_vec(), tz, &mut d)?,
            None => Data::new(&ts_str, fmt, tz, data.as_bytes().to_vec())?,
        };
        // If transform exists modify the value enum and
        match (
            transform,
            serde_json::from_slice::<serde_json::Value>(&data.raw),
        ) {
            (Some(t), Ok(mut v)) => match v.get_mut(&self.field) {
                Some(v_mut) => {
                    let dt = data.timestamp.format(t).to_string();
                    *v_mut = serde_json::Value::String(dt);
                    data.raw = serde_json::to_string(&v)?.as_bytes().to_vec();
                    Ok(data)
                }
                None => {
                    let err = Error {
                        reason: format!(
                            "Timestamp ({}) could not be parsed: {}",
                            &self.field,
                            data.as_string()?
                        ),
                        kind: ErrorKind::Parser,
                    };
                    error!("Error occured during parsing: {:?}", err);
                    Err(err)
                }
            },
            (_, Err(e)) => {
                error!("Error occured during parsing: {:?}", e);
                Err(e.into())
            }
            (None, Ok(_)) => Ok(data),
        }
    }
}

//! # Csv Parser
//!
//! The Csv parser is used to read in data from a Csv source and parse out a date time field to be used in aggregation.
//!

use crate::{
    error::{Error, ErrorKind},
    input::Parser,
    Data, Result,
};

pub struct CsvParser {
    level: u8,
}

impl CsvParser {
    pub fn new(level: u8) -> Self {
        Self { level }
    }
}

impl Parser for CsvParser {
    fn parse_data(
        &self,
        raw: Vec<u8>,
        fmt: Option<&String>,
        tz: Option<&String>,
        dict: Option<&mut crate::FormatDictionary>,
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
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(data.as_bytes());
        if let Some(res) = reader.records().next() {
            let v: csv::StringRecord = res?;
            if let Some(ts_str) = v.get(self.level as usize) {
                let data = match dict {
                    Some(d) => Data::from_dict(&ts_str, raw, tz, d)?,
                    None => Data::new(&ts_str, fmt, tz, raw)?,
                };

                debug!("Parsed data from raw bytes: {:?}", data);
                return Ok(data);
            }
        }
        let err = Error {
            reason: format!("No CSV row in data provided: {}", data),
            kind: ErrorKind::Parser,
        };
        error!("Error occured during parsing: {:?}", err);
        Err(err)
    }
}

//! # Csv Parser
//!
//! The Csv parser is used to read in data from a Csv source and parse out a date time field to be used in aggregation.
//!

// use crate::{
//     error::{Error, ErrorKind},
//     input::Parser,
//     Data, Result,
// };
// use chrono::{DateTime, FixedOffset};

// pub struct CsvParser;

// impl Default for CsvParser {
//     fn default() -> Self {
//         Self
//     }
// }

// impl Parser for CsvParser {
//     fn parse_data(&self, raw: Vec<u8>, field: &str) -> Result<Data<FixedOffset>> {
//         // Parse Data to Json Value and pull value at field provided
//         let field_level
//         let json_value = serde_json::to_value(raw.clone())?;
//         match json_value[field].as_str() {
//             Some(t) => {
//                 // Parse timestamp from field value.
//                 if let Ok(dt) = DateTime::parse_from_rfc2822(t) {
//                     Ok(Data { raw, timestamp: dt })
//                 } else if let Ok(dt) = DateTime::parse_from_rfc3339(t) {
//                     Ok(Data { raw, timestamp: dt })
//                 } else {
//                     Err(Error {
//                         reason: format!("Unable to parse '{}' into a timestamp, please provide an appropriate format.", t),
//                         kind: ErrorKind::Parser,
//                     })
//                 }
//             }
//             None => Err(Error {
//                 reason: format!("Timestamp field '{}' does not exist.", field),
//                 kind: ErrorKind::Parser,
//             }),
//         }
//     }
// }

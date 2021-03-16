//! # RangeAggregator Aggregator
//!
//! The RangeAggregator Aggregator component can be used to find all Data objects that fall in a particular date time range.
//!

use crate::{aggregators::Aggregator, Data, Result};
use chrono::{DateTime, FixedOffset};
use std::fs;

// 1 GB limit to Range Aggregator
const SIZE_LIMIT: usize = 1_000_000;
const TEMP_SAVE: &str = "/tmp/DTA_RANGE_AGG";

pub struct RangeAggregator {
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    in_range: Vec<Data>,
    current_size: usize,
}

impl Aggregator for RangeAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        if data.timestamp >= self.start && data.timestamp <= self.end {
            self.in_range.push(data.clone());
            self.current_size += data.raw.len();
            debug!("Added date to store, date {:?}", data.timestamp);
        }
        if self.current_size > SIZE_LIMIT {
            self.current_size = 0;
            for data in self.in_range.drain(..) {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .read(true)
                    .append(true)
                    .open(TEMP_SAVE)?;
                // Write to file
                use std::io::Write;
                let _ = file.write(&data.raw)?;
                let _ = file.write(b"\n")?;
            }
        }
        Ok(())
    }
}

impl RangeAggregator {
    pub fn new(start: String, end: Option<String>) -> Result<Self> {
        let start_time = Data::new(start.as_str(), None, None, start.as_bytes().to_vec())?;
        let end_time = match end {
            None => {
                let date = chrono::Utc::now().to_string();
                Data::new(date.as_str(), None, None, date.as_bytes().to_vec())?
            }
            Some(e) => Data::new(e.as_str(), None, None, e.as_bytes().to_vec())?,
        };
        Ok(Self {
            in_range: Vec::new(),
            start: start_time.timestamp,
            end: end_time.timestamp,
            current_size: 0,
        })
    }

    pub fn output(&self) -> Result<()> {
        // Read the in_range data and the temp_file data.
        // Print the data.

        Ok(())
    }
}

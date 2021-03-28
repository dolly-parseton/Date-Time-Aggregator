//! # RangeAggregator Aggregator
//!
//! The RangeAggregator Aggregator component can be used to find all Data objects that fall in a particular date time range.
//!

use crate::{aggregators::Aggregator, Data, Result};
use chrono::{DateTime, FixedOffset};
use std::{
    fs,
    io::{prelude::*, BufReader},
};

// 1 GB limit to Range Aggregator
const SIZE_LIMIT: usize = 1_000_000;
const TEMP_SAVE: &str = "/tmp/DTA_RANGE_AGG";

pub struct RangeAggregator {
    start: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    inverted: bool,
    in_range: Vec<Data>,
    current_size: usize,
    data_written_to_file: bool,
}

impl Aggregator for RangeAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        if (data.timestamp >= self.start && data.timestamp <= self.end && !self.inverted)
            || ((data.timestamp <= self.start || data.timestamp >= self.end) && self.inverted)
        {
            self.in_range.push(data.clone());
            self.current_size += data.raw.len();
            debug!("Added date to store, date {:?}", data.timestamp);
        }
        if self.current_size > SIZE_LIMIT {
            self.current_size = 0;
            for data in self.in_range.drain(..) {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(TEMP_SAVE)?;
                // Write to file
                let _ = file.write(&data.raw)?;
                let _ = file.write(b"\n")?;
                self.data_written_to_file = true;
            }
        }
        Ok(())
    }
    fn return_value(&self) -> Result<String> {
        //
        for data in &self.in_range {
            println!("{}", data.as_string()?.trim_end());
        }
        if self.data_written_to_file {
            let file = fs::OpenOptions::new().read(true).open(TEMP_SAVE)?;
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            while let Ok(0) = reader.read_line(&mut line) {
                println!("{}", line.trim_end());
            }
            fs::remove_file(TEMP_SAVE)?;
        }
        Ok("".to_string())
    }
}

impl RangeAggregator {
    pub fn new(start: String, end: Option<String>, inverted: bool) -> Result<Self> {
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
            inverted,
            current_size: 0,
            data_written_to_file: false,
        })
    }
}

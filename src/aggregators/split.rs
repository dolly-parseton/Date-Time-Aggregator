//! # Maximum Aggregator
//!
//! The Maximum Aggregator component can be used to find the entry with the most recent timestamp.
//!

use crate::{aggregators::Aggregator, error, Data, Result};
use std::{convert::TryFrom, fs, path::PathBuf};

pub struct SplitAggregator {
    increment: Increment,
    date_as_prefix: bool,
    flatten: bool,
    output_directory: PathBuf,
}

enum Increment {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Timezone,
}

impl TryFrom<&str> for Increment {
    type Error = error::Error;
    fn try_from(s: &str) -> Result<Increment> {
        match s.to_ascii_lowercase().as_str() {
            "year" => Ok(Increment::Year),
            "month" => Ok(Increment::Month),
            "day" => Ok(Increment::Day),
            "hour" => Ok(Increment::Hour),
            "minute" => Ok(Increment::Minute),
            "second" => Ok(Increment::Second),
            "timezone" => Ok(Increment::Timezone),
            _ => Err(error::Error {
                reason: "Invalid split format provided.".to_string(),
                kind: error::ErrorKind::Aggregator,
            }),
        }
    }
}

impl Aggregator for SplitAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        // Using the Level Enum do a custom format based on the level and use that string in creating a file prefix
        let formatted = match self.increment {
            Increment::Year => data.timestamp.format("%y"),
            Increment::Month => data.timestamp.format(match self.flatten {
                true => "%m",
                false => "%y-%m",
            }),
            Increment::Day => data.timestamp.format(match self.flatten {
                true => "%d",
                false => "%y-%m-%d",
            }),
            Increment::Hour => data.timestamp.format(match self.flatten {
                true => "%H",
                false => "%y-%m-%dT%H",
            }),
            Increment::Minute => data.timestamp.format(match self.flatten {
                true => "%M",
                false => "%y-%m-%dT%H:%M",
            }),
            Increment::Second => data.timestamp.format(match self.flatten {
                true => "%S",
                false => "%y-%m-%dT%H:%M:%S",
            }),
            Increment::Timezone => data.timestamp.format("%z"),
        }
        .to_string();
        // Append data to file
        let file_name = self.output_directory.join(match self.date_as_prefix {
            true => format!("{}_dta", formatted),
            false => format!("dta_{}", formatted),
        });
        let mut file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&file_name)?;
        // Write to file
        use std::io::Write;
        let len = file.write(&data.raw)?;
        let _ = file.write(b"\n")?;
        //
        debug!(
            "Written {} bytes to {}/{}",
            len,
            self.output_directory.display(),
            file_name.display()
        );
        Ok(())
    }
}

impl SplitAggregator {
    pub fn new(
        increment: String,
        date_as_prefix: bool,
        flatten: bool,
        output_directory: PathBuf,
    ) -> Result<Self> {
        Ok(Self {
            increment: Increment::try_from(increment.as_str())?,
            date_as_prefix,
            flatten,
            output_directory,
        })
    }

    pub fn output(&self) -> Result<()> {
        // debug!("Maximum Aggregator returning output: {:?}", self.largest);
        // Ok(self.largest.clone())
        Ok(())
    }
}

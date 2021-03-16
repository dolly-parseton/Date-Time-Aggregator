//! # Split Aggregator
//!
//! The Split Aggregator component can be used to split data being provided into different date increments.
//! Options for the increment enum include:
//! * Year
//! * Month
//! * Day
//! * Hour
//! * Minute
//! * Second
//! * Timezone
//! The increment option of the [`SplitAggregator::new()`](SplitAggregator::new()) function accepts a string of any case matching the above options.
//! [`SplitAggregator::new()`](SplitAggregator::new()) also accepts and option to flatten the resulting data so data with a timestamp of 2021-01-01 01:00:00 with a split increment of "month" will be saved to a file called "./output_directory/01_dta".
use crate::{aggregators::Aggregator, Data, Result};
use std::{fs, path::PathBuf};

pub struct SplitAggregator {
    output_directory: PathBuf,
    filename: String,
}

impl Aggregator for SplitAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        let path = self
            .output_directory
            .join(data.timestamp.format(&self.filename).to_string());
        if self.filename.contains('/') {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(&parent)?;
            }
        }
        let mut file = fs::OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&path)?;
        // Write to file
        use std::io::Write;
        let len = file.write(&data.raw)?;
        let _ = file.write(b"\n")?;
        //
        debug!(
            "Written {} bytes to {}/{}",
            len,
            self.output_directory.display(),
            &self
                .output_directory
                .join(data.timestamp.format(&self.filename).to_string())
                .display()
        );
        Ok(())
    }
}

impl SplitAggregator {
    /// Function to create a `SplitAggregator`, provide:
    /// * A string that'll be parsed into an `Increment` (Options listed in module level documentation).
    /// * A boolean to determine if the filename string is a prefix or suffix to resulting files.
    /// * A boolean to determine if the dates are flattened in the filename.
    pub fn new(output_directory: PathBuf, filename: String) -> Result<Self> {
        Ok(Self {
            output_directory,
            filename,
        })
    }

    /// Return the output of the aggregation
    pub fn output(&self) -> Result<()> {
        // debug!("Maximum Aggregator returning output: {:?}", self.largest);
        // Ok(self.largest.clone())
        Ok(())
    }
}

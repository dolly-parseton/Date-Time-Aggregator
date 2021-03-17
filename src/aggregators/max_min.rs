//! # Maximum Aggregator
//!
//! The Maximum Aggregator component can be used to find the entry with the most recent timestamp.
//!

use crate::{aggregators::Aggregator, error, Data, Result};

pub struct MaximumAggregator {
    pub largest: Option<Data>,
}

impl Default for MaximumAggregator {
    fn default() -> Self {
        Self { largest: None }
    }
}

impl Aggregator for MaximumAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        if let Some(largest) = &self.largest {
            if largest.timestamp < data.timestamp {
                self.largest = Some(data.clone());
                debug!("Updated Maximum Aggregator State: {:?}", self.largest);
            }
        } else {
            self.largest = Some(data.clone());
        }
        Ok(())
    }
    fn return_value(&self) -> Result<String> {
        match self.output()? {
            Some(d) => d.as_string(),
            None => Err(error::Error {
                reason: "Aggregator did not return a value".to_string(),
                kind: error::ErrorKind::Aggregator,
            }),
        }
    }
}

impl MaximumAggregator {
    pub fn output(&self) -> Result<Option<Data>> {
        debug!("Maximum Aggregator returning output: {:?}", self.largest);
        Ok(self.largest.clone())
    }
}

//

pub struct MinimumAggregator {
    pub smallest: Option<Data>,
}

impl Default for MinimumAggregator {
    fn default() -> Self {
        Self { smallest: None }
    }
}

impl Aggregator for MinimumAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        if let Some(smallest) = &self.smallest {
            if smallest.timestamp < data.timestamp {
                self.smallest = Some(data.clone());
                debug!("Updated Maximum Aggregator State: {:?}", self.smallest);
            }
        } else {
            self.smallest = Some(data.clone());
        }
        Ok(())
    }
    fn return_value(&self) -> Result<String> {
        match self.output()? {
            Some(d) => d.as_string(),
            None => Err(error::Error {
                reason: "Aggregator did not return a value".to_string(),
                kind: error::ErrorKind::Aggregator,
            }),
        }
    }
}

impl MinimumAggregator {
    pub fn output(&self) -> Result<Option<Data>> {
        debug!("Minimum Aggregator returning output: {:?}", self.smallest);
        Ok(self.smallest.clone())
    }
}

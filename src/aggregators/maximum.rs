//! # Maximum Aggregator
//!
//! The Maximum Aggregator component can be used to find the entry with the most recent timestamp.
//!

use crate::{aggregators::Aggregator, Data, Result};

pub struct MaximumAggregator {
    pub largest: Data,
}

impl Default for MaximumAggregator {
    fn default() -> Self {
        Self {
            largest: Data::default(),
        }
    }
}

impl Aggregator for MaximumAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        if self.largest.timestamp < data.timestamp {
            self.largest = data.clone();
            debug!("Updated Maximum Aggregator State: {:?}", self.largest);
        }
        Ok(())
    }

    fn output(&self) -> Result<Data> {
        debug!("Maximum Aggregator returning output: {:?}", self.largest);
        Ok(self.largest.clone())
    }
}

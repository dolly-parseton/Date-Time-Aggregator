//! # Maximum Aggregator
//!
//! The Maximum Aggregator component can be used to find the entry with the most recent timestamp.
//!

use crate::{
    aggregators::{Aggregator, Increment},
    error, Data, Result,
};
use chrono::{DateTime, FixedOffset};
use std::{collections::HashMap, convert::TryFrom};

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

pub struct MaximumsAggregator {
    pub largests: HashMap<DateTime<FixedOffset>, Data>,
    pub increment: Increment,
}

impl Aggregator for MaximumsAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        let rounded = self.increment.rounded(data.timestamp.clone())?;

        match self.largests.remove(&rounded) {
            Some(l) => {
                if l.timestamp <= data.timestamp {
                    self.largests.insert(rounded, data.clone());
                }
            }
            None => {
                self.largests.insert(rounded, data.clone());
            }
        };
        Ok(())
    }
    fn return_value(&self) -> Result<String> {
        let mut pretty: String = String::new();
        for (_k, v) in self.largests.iter() {
            pretty.push_str(&format!("\n{}", v.as_string()?));
        }
        Ok(format!("Maximums for increment: {}", pretty))
    }
}

impl MaximumsAggregator {
    pub fn new(increment: String) -> Result<Self> {
        Ok(Self {
            increment: Increment::try_from(increment)?,
            largests: HashMap::new(),
        })
    }
    pub fn output(&self) -> Result<HashMap<DateTime<FixedOffset>, Data>> {
        debug!("Maximum Aggregator returning output: {:#?}", self.largests);
        Ok(self.largests.clone())
    }
}

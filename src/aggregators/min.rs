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

//

pub struct MinimumsAggregator {
    pub smallests: HashMap<DateTime<FixedOffset>, Data>,
    pub increment: Increment,
}

impl Aggregator for MinimumsAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        let rounded = self.increment.rounded(data.timestamp)?;

        match self.smallests.remove(&rounded) {
            Some(s) => {
                if s.timestamp >= data.timestamp {
                    self.smallests.insert(rounded, data.clone());
                }
            }
            None => {
                self.smallests.insert(rounded, data.clone());
            }
        };
        Ok(())
    }
    fn return_value(&self) -> Result<String> {
        let mut pretty: String = String::new();
        for (_k, v) in self.smallests.iter() {
            pretty.push_str(&format!("\n{}", v.as_string()?));
        }
        Ok(format!("Minimums for increment: {}", pretty))
    }
}

impl MinimumsAggregator {
    pub fn new(increment: String) -> Result<Self> {
        Ok(Self {
            increment: Increment::try_from(increment)?,
            smallests: HashMap::new(),
        })
    }
    pub fn output(&self) -> Result<HashMap<DateTime<FixedOffset>, Data>> {
        debug!(
            "Minimums Aggregator returning output: {:#?}",
            self.smallests
        );
        Ok(self.smallests.clone())
    }
}

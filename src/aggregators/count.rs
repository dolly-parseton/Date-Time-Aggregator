use crate::{
    aggregators::{Aggregator, Increment},
    Data, Result,
};
use chrono::{DateTime, FixedOffset};
use std::{collections::HashMap, convert::TryFrom};

pub struct CountAggregator {
    pub n: u64,
}
impl Default for CountAggregator {
    fn default() -> Self {
        Self { n: 0 }
    }
}
impl Aggregator for CountAggregator {
    fn update(&mut self, _data: &Data) -> Result<()> {
        self.n += 1;
        debug!("Updated Maximum Aggregator State: {:?}", self.n);
        Ok(())
    }
    fn return_value(&self) -> Result<String> {
        Ok(format!("Count: {}", self.n))
    }
}

impl CountAggregator {
    pub fn output(&self) -> Result<u64> {
        debug!("Maximum Aggregator returning output: {:?}", self.n);
        Ok(self.n)
    }
}

//

pub struct CountsAggregator {
    pub counts: HashMap<DateTime<FixedOffset>, u64>,
    pub increment: Increment,
}

impl Aggregator for CountsAggregator {
    fn update(&mut self, data: &Data) -> Result<()> {
        let rounded = self.increment.rounded(data.timestamp.clone())?;

        match self.counts.remove(&rounded) {
            Some(c) => self.counts.insert(rounded, c + 1),
            None => self.counts.insert(rounded, 1),
        };
        Ok(())
    }
    fn return_value(&self) -> Result<String> {
        let mut pretty: String = String::new();
        for (k, v) in self.counts.iter() {
            pretty.push_str(&format!("\n{}: {}", k.with_timezone(&chrono::Utc), v));
        }
        Ok(format!("Maximums for increment: {}", pretty))
    }
}

impl CountsAggregator {
    pub fn new(increment: String) -> Result<Self> {
        Ok(Self {
            increment: Increment::try_from(increment)?,
            counts: HashMap::new(),
        })
    }
    pub fn output(&self) -> Result<HashMap<DateTime<FixedOffset>, u64>> {
        debug!("Maximum Aggregator returning output: {:#?}", self.counts);
        Ok(self.counts.clone())
    }
}

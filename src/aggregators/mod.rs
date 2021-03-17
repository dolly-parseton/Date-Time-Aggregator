//! # Aggregators
//!
//! The aggregator functions are a key part of `dta`. Some aggregation functions include:
//!
//! * WIP:
//! ** Range
//! ** Split
//! ** Maximum
//! ** Minimum
//!
//! Note: Not selecting an aggregator when running the `dta` binary will pass the data along
//!
pub mod max_min;
pub mod range;
pub mod split;

use crate::{Data, Result};

pub trait Aggregator {
    fn update(&mut self, data: &Data) -> Result<()>;
    fn return_value(&self) -> Result<String>;
}

pub mod count {
    use super::*;
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
}

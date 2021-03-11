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
pub mod maximum;

use crate::{Data, Result};
use chrono::{DateTime, FixedOffset};

pub trait Aggregator {
    // Update the state of the aggregation.
    fn update(&mut self, data: &Data) -> Result<()>;

    /// Return the output of the aggregation.
    fn output(&self) -> Result<Data>;
}

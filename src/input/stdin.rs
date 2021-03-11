//! # Stdin Source
//!
//! The Stdin source component is used to read in data from current process input stream.
//!

use crate::{input::Source, Result};
use std::io::{self, BufRead};

pub struct StdinSource {
    /// Stdin for current process
    pub stdin: io::Stdin,
}

impl Default for StdinSource {
    fn default() -> Self {
        Self { stdin: io::stdin() }
    }
}

impl Source for StdinSource {
    fn read_data(&self) -> Result<Vec<u8>> {
        match self.stdin.lock().lines().next() {
            Some(Ok(input)) => {
                debug!("Reading {} bytes from Stdin: \"{}\"", input.len(), input);
                Ok(input.as_bytes().to_vec())
            }
            _ => Ok(Vec::new()),
        }
    }
}

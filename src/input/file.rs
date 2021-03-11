//! # File Source
//!
//! The file source component is used to read in a file or multiple files based on a glob.
//!

use crate::{input::Source, Result};

pub struct FileSource;

impl Default for FileSource {
    fn default() -> Self {
        Self
    }
}

impl Source for FileSource {
    fn read_data(&self) -> Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

//! # File Source
//!
//! The file source component is used to read in a file or multiple files based on a glob.
//!

use crate::{error, input::Source, Result};
use std::{
    fs,
    io::{prelude::*, BufReader},
};

pub struct FileSource {
    glob: glob::Paths,
    current_reader: BufReader<fs::File>,
}

impl FileSource {
    pub fn new(glob_str: &str, case_sensitive: bool) -> Result<Self> {
        let mut glob = glob::glob_with(
            glob_str,
            glob::MatchOptions {
                case_sensitive,
                require_literal_separator: false,
                require_literal_leading_dot: false,
            },
        )?;
        let first_path = match glob.next() {
            Some(p) => p,
            None => {
                return Err(error::Error {
                    reason: format!("The glob provided ({}) did not return any paths.", glob_str),
                    kind: error::ErrorKind::Input,
                })
            }
        };
        let file = fs::OpenOptions::new().read(true).open(first_path?)?;
        let current_reader = BufReader::new(file);
        Ok(Self {
            glob,
            current_reader,
        })
    }
}

impl Source for FileSource {
    fn read_data(&mut self) -> Result<Vec<u8>> {
        let mut line = String::new();
        match self.current_reader.read_line(&mut line) {
            Ok(len) => {
                debug!("Reading {} bytes from Stdin: \"{}\"", len, line);
                // Return data
                Ok(line.as_bytes().to_vec())
            }
            Err(_e) => {
                match self.glob.next() {
                    Some(p) => {
                        // Create a BufReader
                        let file = fs::OpenOptions::new().read(true).open(p?)?;
                        let mut reader = BufReader::new(file);
                        // Read line
                        let mut line = String::new();
                        let len = reader.read_line(&mut line)?;
                        debug!("Reading {} bytes from Stdin: \"{}\"", len, line);
                        // Store reader
                        self.current_reader = reader;
                        // Return Data
                        Ok(line.as_bytes().to_vec())
                    }
                    None => Ok(Vec::new()),
                }
            }
        }

        //     if let Some(_) = &self.current_reader {
        //         // Get reader
        //         let mut reader = self.current_reader.unwrap();
        //         // Read line
        //         let mut line = String::new();
        //         let len = reader.read_line(&mut line)?;
        //         debug!("Reading {} bytes from Stdin: \"{}\"", len, line);
        //         // Store reader
        //         self.current_reader = Some(reader);
        //         // Return Data
        //         Ok(line.as_bytes().to_vec())
        //     } else {
        //         match self.glob.next() {
        //             Some(p) => {
        //                 // Create a BufReader
        //                 let file = fs::OpenOptions::new().read(true).open(p?)?;
        //                 let mut reader = BufReader::new(file);
        //                 // Read line
        //                 let mut line = String::new();
        //                 let len = reader.read_line(&mut line)?;
        //                 debug!("Reading {} bytes from Stdin: \"{}\"", len, line);
        //                 // Store reader
        //                 self.current_reader = Some(reader);
        //                 // Return Data
        //                 Ok(line.as_bytes().to_vec())
        //             }
        //             None => return Ok(Vec::new()),
        //         }
        //     }
    }
}

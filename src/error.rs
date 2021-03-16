//! # DTA Errors
//!
//!

// Crate level result struct wrapping the error enum
pub type Result<T> = std::result::Result<T, Error>;
use std::{error::Error as ErrTrait, fmt};

// Crate level error struct
#[derive(Debug, Clone)]
pub struct Error {
    pub reason: String,
    pub kind: ErrorKind,
}

// Crate level error kind enum
#[derive(Debug, Clone)]
pub enum ErrorKind {
    Data { raw: Vec<u8> },
    Parser,
    DateTime,
    Timezone,
    Aggregator,
    Input,
}

impl ErrTrait for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.reason)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

impl From<glob::GlobError> for Error {
    fn from(err: glob::GlobError) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

impl From<std::num::TryFromIntError> for Error {
    fn from(err: std::num::TryFromIntError) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::DateTime,
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Timezone,
        }
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(err: chrono::format::ParseError) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::DateTime,
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::DateTime,
        }
    }
}

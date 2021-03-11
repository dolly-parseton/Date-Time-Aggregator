//! # DTA Errors
//!
//!

// Crate level result struct wrapping the error enum
pub type Result<T> = std::result::Result<T, Error>;

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
    Input,
}

#[cfg(feature = "json-parser")]
impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

#[cfg(feature = "csv-parser")]
impl From<csv::Error> for Error {
    fn from(err: csv::Error) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

#[cfg(feature = "file-input")]
impl From<glob::PatternError> for Error {
    fn from(err: glob::PatternError) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

#[cfg(feature = "file-input")]
impl From<glob::GlobError> for Error {
    fn from(err: glob::GlobError) -> Self {
        Self {
            reason: format!("{}", err),
            kind: ErrorKind::Parser,
        }
    }
}

#[cfg(feature = "file-input")]
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

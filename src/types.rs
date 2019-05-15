//! # types
//!
//! Common library types.
//!

use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
/// Holds the error information
pub enum ErrorInfo {
    /// File not found error
    FileNotFound(&'static str),
    /// File open error
    FileOpen(&'static str),
}

#[derive(Debug, Copy, Clone)]
/// Error struct
pub struct EnvmntError {
    /// Holds the error information
    pub info: ErrorInfo,
}

impl Error for EnvmntError {
    /// A short description of the error.
    fn description(&self) -> &str {
        match self.info {
            ErrorInfo::FileNotFound(description) => description,
            ErrorInfo::FileOpen(description) => description,
        }
    }

    /// The lower-level cause of this error, if any.
    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl Display for EnvmntError {
    /// Formats the value using the given formatter.
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::FileNotFound(ref file) => file.fmt(format),
            ErrorInfo::FileOpen(ref file) => file.fmt(format),
        }
    }
}

//! # errors
//!
//! Common library errors.
//!

#[cfg(test)]
#[path = "./errors_test.rs"]
mod errors_test;

use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
/// Holds the error information
pub enum ErrorInfo {
    /// File not found error
    FileNotFound(&'static str),
    /// File open error
    FileOpen(Error),
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
            ErrorInfo::FileOpen(ref cause) => cause.description(),
        }
    }

    /// The lower-level cause of this error, if any.
    fn cause(&self) -> Option<&error::Error> {
        match self.info {
            ErrorInfo::FileOpen(ref cause) => Some(cause),
            _ => None,
        }
    }
}

impl Display for EnvmntError {
    /// Formats the value using the given formatter.
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::FileNotFound(ref file) => file.fmt("File: {} not found.", format),
            ErrorInfo::FileOpen(ref cause) => cause.fmt(format),
        }
    }
}

impl EnvmntError {
    pub fn is_file_not_found(&self) -> bool {
        match self.info {
            ErrorInfo::FileNotFound(_) => true,
            _ => false,
        }
    }

    pub fn is_file_open(&self) -> bool {
        match self.info {
            ErrorInfo::FileOpen(_) => true,
            _ => false,
        }
    }
}

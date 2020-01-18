//! # errors
//!
//! Common library errors.
//!

#[cfg(test)]
#[path = "./errors_test.rs"]
mod errors_test;

use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
/// Holds the error information
pub enum ErrorKind {
    /// File not found error
    FileNotFound(&'static str),
    /// File open error
    FileOpen(&'static str),
}

#[derive(Debug, Copy, Clone)]
/// Error struct
pub struct EnvmntError {
    /// Holds the error information
    pub kind: ErrorKind,
}

impl Display for EnvmntError {
    /// Formats the value using the given formatter.
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.kind {
            ErrorKind::FileNotFound(ref file) => file.fmt(format),
            ErrorKind::FileOpen(ref file) => file.fmt(format),
        }
    }
}

impl EnvmntError {
    pub fn is_file_not_found(&self) -> bool {
        match self.kind {
            ErrorKind::FileNotFound(_) => true,
            _ => false,
        }
    }

    pub fn is_file_open(&self) -> bool {
        match self.kind {
            ErrorKind::FileOpen(_) => true,
            _ => false,
        }
    }
}

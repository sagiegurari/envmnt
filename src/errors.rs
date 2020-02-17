//! # errors
//!
//! Common library errors.
//!

#[cfg(test)]
#[path = "./errors_test.rs"]
mod errors_test;

use fsio::error::FsIOError;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
/// Holds the error information
pub enum ErrorKind {
    /// File not found error
    ReadFile(&'static str, FsIOError),
}

#[derive(Debug)]
/// Error struct
pub struct EnvmntError {
    /// Holds the error information
    pub kind: ErrorKind,
}

impl Display for EnvmntError {
    /// Formats the value using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.kind {
            ErrorKind::ReadFile(ref message, ref cause) => {
                writeln!(formatter, "{}", message)?;
                cause.fmt(formatter)
            }
        }
    }
}

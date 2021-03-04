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

/// Envmt Library Result
pub type EnvmntResult<T> = Result<T, EnvmntError>;

#[derive(Debug)]
/// Error struct
pub enum EnvmntError {
    /// File not found error
    ReadFile(&'static str, FsIOError),
    /// Environment variable not found
    Missing(String),
    /// Environment variable has not a compatible type
    InvalidType(String),
}

impl Display for EnvmntError {
    /// Formats the value using the given formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::ReadFile(ref message, ref cause) => {
                writeln!(formatter, "{}", message)?;
                cause.fmt(formatter)
            }
            Self::Missing(ref msg) => writeln!(formatter, "{}", msg),
            Self::InvalidType(ref msg) => writeln!(formatter, "{}", msg),
        }
    }
}

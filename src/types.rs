//! # types
//!
//! Common library types.
//!

use failure::Fail;
use std::io::Error;

#[derive(Debug, Fail)]
pub enum EnvmntError {
    #[fail(display = "File: {} not found", file)]
    FileNotFound { file: String },
    #[fail(display = "Unable to open file: {}", file)]
    FileOpen { file: String, cause: Error },
}

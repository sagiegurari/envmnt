//! # generic
//!
//! Environment variable getter over any type T as long as it implements `std::str::FromStr`.
//!

#[cfg(test)]
#[path = "./generic_tests.rs"]
mod generic_tests;

use crate::errors::EnvmntError;
use crate::errors::ErrorKind;
use std::env::var;
use std::env::VarError;
use std::ffi::OsStr;
use std::fmt::Display;
use std::str::FromStr;

pub(crate) fn get_parse<K, T, E>(key: K) -> Result<T, EnvmntError>
where
    K: AsRef<OsStr>,
    T: FromStr + FromStr<Err = E>,
    E: Display,
{
    match var(&key) {
        Ok(valstr) => valstr.parse::<T>().map_err(|e| EnvmntError {
            kind: ErrorKind::InvalidType(format!(
                "Environment variable {} is of incompatible type {}. Failed to parse with: {}",
                key.as_ref().to_string_lossy(),
                stringify!(T),
                e
            )),
        }),

        Err(VarError::NotPresent) => Err(EnvmntError {
            kind: ErrorKind::Missing(format!(
                "Environment variable '{}' is not set",
                key.as_ref().to_string_lossy()
            )),
        }),

        Err(VarError::NotUnicode(osstr)) => Err(EnvmntError {
            kind: ErrorKind::InvalidType(format!(
                "Environment variable is not valid unicode: {:#?}",
                osstr
            )),
        }),
    }
}

pub(crate) fn get_parse_or<K, T, E>(key: K, default: T) -> Result<T, EnvmntError>
where
    K: AsRef<OsStr>,
    T: FromStr + FromStr<Err = E>,
    E: Display,
{
    let result = get_parse(key);

    if let Err(EnvmntError {
        kind: ErrorKind::Missing(_),
    }) = result
    {
        Ok(default)
    } else {
        result
    }
}

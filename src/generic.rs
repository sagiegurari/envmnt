//! # generic
//!
//! Environment variable getter over any type T as long as it implements `std::str::FromStr`.
//!

#[cfg(test)]
#[path = "./generic_tests.rs"]
mod generic_tests;

use crate::errors::EnvmntError;
use crate::types::EnvmntResult;
use std::env::var;
use std::env::VarError;
use std::ffi::OsStr;
use std::fmt::Display;
use std::str::FromStr;

pub(crate) fn get_parse<K, T, E>(key: K) -> EnvmntResult<T>
where
    K: AsRef<OsStr>,
    T: FromStr + FromStr<Err = E>,
    E: Display,
{
    match var(&key) {
        Ok(valstr) => valstr.parse::<T>().map_err(|error| {
            EnvmntError::InvalidType(format!(
                "Environment variable {} is of incompatible type {}. Failed to parse with: {}",
                key.as_ref().to_string_lossy(),
                stringify!(T),
                error
            ))
        }),

        Err(VarError::NotPresent) => Err(EnvmntError::Missing(format!(
            "Environment variable '{}' is not set",
            key.as_ref().to_string_lossy()
        ))),

        Err(VarError::NotUnicode(osstr)) => Err(EnvmntError::InvalidType(format!(
            "Environment variable is not valid unicode: {:#?}",
            osstr
        ))),
    }
}

pub(crate) fn get_parse_or<K, T, E>(key: K, default: T) -> EnvmntResult<T>
where
    K: AsRef<OsStr>,
    T: FromStr + FromStr<Err = E>,
    E: Display,
{
    let result = get_parse(key);

    if let Err(EnvmntError::Missing(_)) = result {
        Ok(default)
    } else {
        result
    }
}

//! # environment
//!
//! Environment variables direct access functions.
//!

#[cfg(test)]
#[path = "./environment_test.rs"]
mod environment_test;

use crate::util;
use std::env;

pub(crate) fn exists(key: &str) -> bool {
    match env::var(key) {
        Ok(_) => true,
        _ => false,
    }
}

pub(crate) fn remove(key: &str) {
    env::remove_var(key)
}

pub(crate) fn get_remove(key: &str) -> Option<String> {
    let pre_value = if exists(key) {
        Some(get_or(key, ""))
    } else {
        None
    };

    remove(key);

    pre_value
}

pub(crate) fn get_or(key: &str, default_value: &str) -> String {
    match env::var(key) {
        Ok(value) => value.to_string(),
        _ => default_value.to_string(),
    }
}

pub(crate) fn is_or(key: &str, default_value: bool) -> bool {
    let default_str = util::bool_to_string(default_value);

    let value = get_or(key, &default_str);

    util::string_to_bool(&value)
}

pub(crate) fn set(key: &str, value: &str) {
    env::set_var(&key, &value);
}

pub(crate) fn set_bool(key: &str, value: bool) {
    let value_str = util::bool_to_string(value);
    set(key, &value_str);
}

pub(crate) fn get_set(key: &str, value: &str) -> Option<String> {
    let pre_value = if exists(key) {
        Some(get_or(key, ""))
    } else {
        None
    };

    env::set_var(&key, &value);

    pre_value
}

//! # environment
//!
//! Environment variables direct access functions.
//!

#[cfg(test)]
#[path = "./environment_test.rs"]
mod environment_test;

use crate::util;
use std::env;
use std::ffi::OsStr;

pub(crate) fn exists<K: AsRef<OsStr>>(key: K) -> bool {
    match env::var(key) {
        Ok(_) => true,
        _ => false,
    }
}

pub(crate) fn remove<K: AsRef<OsStr>>(key: K) {
    env::remove_var(key)
}

pub(crate) fn get_remove<K: AsRef<OsStr>>(key: K) -> Option<String> {
    let pre_value = if exists(&key) {
        Some(get_or(&key, ""))
    } else {
        None
    };

    remove(key);

    pre_value
}

pub(crate) fn get_or<K: AsRef<OsStr>>(key: K, default_value: &str) -> String {
    match env::var(key) {
        Ok(value) => value.to_string(),
        _ => default_value.to_string(),
    }
}

pub(crate) fn get_or_panic<K: AsRef<OsStr>>(key: K) -> String {
    env::var(key).unwrap()
}

pub(crate) fn is_or<K: AsRef<OsStr>>(key: K, default_value: bool) -> bool {
    let default_str = util::bool_to_string(default_value);

    let value = get_or(key, &default_str);

    util::string_to_bool(&value)
}

pub(crate) fn set<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    env::set_var(&key, &value);
}

pub(crate) fn set_bool<K: AsRef<OsStr>>(key: K, value: bool) {
    let value_str = util::bool_to_string(value);
    set(key, &value_str);
}

pub(crate) fn get_set<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) -> Option<String> {
    let pre_value = if exists(&key) {
        Some(get_or(&key, ""))
    } else {
        None
    };

    env::set_var(&key, &value);

    pre_value
}

pub(crate) fn vars() -> Vec<(String, String)> {
    env::vars().collect()
}

pub(crate) fn is_equal<K: AsRef<OsStr>>(key: K, value: &str) -> bool {
    if exists(&key) {
        let current_value = get_or(&key, "");

        current_value == value
    } else {
        false
    }
}

//! # bulk
//!
//! Wrapper for env related functions which work on bulk of variables at once.
//!

#[cfg(test)]
#[path = "./bulk_test.rs"]
mod bulk_test;

use crate::environment;
use indexmap::IndexMap;
use std::ffi::OsStr;

pub(crate) fn remove_all<K: AsRef<OsStr>>(keys: &Vec<K>) {
    for key in keys.iter() {
        environment::remove(key);
    }
}

pub(crate) fn set_all(env: &IndexMap<String, String>) {
    for (key, value) in env.iter() {
        environment::set(key, value);
    }
}

pub(crate) fn evaluate_and_set_all<F>(env: &IndexMap<String, String>, evaluate: F)
where
    F: Fn(String, String) -> Option<(String, String)>,
{
    for (key, value) in env.iter() {
        if let Some((updated_key, updated_value)) = evaluate(key.to_string(), value.to_string()) {
            environment::set(updated_key, updated_value);
        }
    }
}

pub(crate) fn is_any_exists<K: AsRef<OsStr>>(keys: &Vec<K>) -> bool {
    let mut found = false;

    for key in keys.iter() {
        found = environment::exists(key);

        if found {
            break;
        }
    }

    found
}

pub(crate) fn is_all_exists<K: AsRef<OsStr>>(keys: &Vec<K>) -> bool {
    let mut found = false;

    for key in keys.iter() {
        found = environment::exists(key);

        if !found {
            break;
        }
    }

    found
}

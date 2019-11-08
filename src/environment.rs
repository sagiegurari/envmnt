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

static UNIX_ENV_SYMBOL: &str = "$";
static UNIX_ENV_PREFIX: &str = "${";
static UNIX_ENV_SUFFIX: &str = "}";
static WINDOWS_ENV_SYMBOL: &str = "%";

/// Get/Set list options
#[derive(Debug, Clone)]
pub struct ListOptions {
    /// The separator used to merge/split the values
    pub separator: Option<String>,
    /// if true, empty list will not be set and empty string will be considered as a list with a single empty value
    pub ignore_empty: bool,
}

impl ListOptions {
    /// Creates and returns a new instance.
    pub fn new() -> ListOptions {
        ListOptions {
            separator: None,
            ignore_empty: false,
        }
    }
}

/// Expansion Type - unix/windows style
#[derive(Debug, Copy, Clone)]
pub enum ExpansionType {
    /// Unix prefix environment style, for example: $MY_ENV
    UnixPrefix,
    /// Unix brackets environment style, for example: ${MY_ENV}
    UnixBrackets,
    /// All unix supported styles
    Unix,
    /// Windows environment style, for example: %MY_ENV%
    Windows,
    /// Current OS supported styles (UnixAll/Windows)
    OS,
    /// All supported styles for all platforms
    All,
}

fn get_os_expansion_type() -> ExpansionType {
    if cfg!(windows) {
        ExpansionType::Windows
    } else {
        ExpansionType::Unix
    }
}

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

pub(crate) fn get_any<K: AsRef<OsStr>>(keys: &Vec<K>, default_value: &str) -> String {
    let mut output = default_value.to_string();

    for key in keys.iter() {
        let current_value = env::var(key);

        if current_value.is_ok() {
            output = current_value.unwrap();
            break;
        }
    }

    output
}

pub(crate) fn is_or<K: AsRef<OsStr>>(key: K, default_value: bool) -> bool {
    let default_str = util::bool_to_string(default_value);

    let value = get_or(key, &default_str);

    util::string_to_bool(&value)
}

pub(crate) fn is<K: AsRef<OsStr>>(key: K) -> bool {
    is_or(&key, false)
}

pub(crate) fn set<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    env::set_var(&key, &value);
}

pub(crate) fn set_bool<K: AsRef<OsStr>>(key: K, value: bool) {
    let value_str = util::bool_to_string(value);
    set(key, &value_str);
}

pub(crate) fn set_optional<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: &Option<V>) -> bool {
    match value {
        Some(ref value_ref) => {
            set(key, value_ref);
            true
        }
        None => false,
    }
}

pub(crate) fn get_set<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) -> Option<String> {
    let pre_value = if exists(&key) {
        Some(get_or(&key, ""))
    } else {
        None
    };

    set(&key, &value);

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

pub(crate) fn contains<K: AsRef<OsStr>>(key: K, value: &str) -> bool {
    if exists(&key) {
        let current_value = get_or(&key, "");

        current_value.contains(value)
    } else {
        false
    }
}

pub(crate) fn contains_ignore_case<K: AsRef<OsStr>>(key: K, value: &str) -> bool {
    if exists(&key) {
        let current_value = get_or(&key, "").to_lowercase();

        current_value.contains(&value.to_lowercase())
    } else {
        false
    }
}

pub(crate) fn set_list<K: AsRef<OsStr>>(key: K, values: &Vec<String>) {
    let options = ListOptions::new();
    set_list_with_options(key, values, &options)
}

pub(crate) fn get_list<K: AsRef<OsStr>>(key: K) -> Option<Vec<String>> {
    let options = ListOptions::new();
    get_list_with_options(key, &options)
}

fn get_list_separator(options: &ListOptions) -> String {
    match options.separator {
        Some(ref separator) => separator.to_string(),
        None => ";".to_string(),
    }
}

pub(crate) fn set_list_with_options<K: AsRef<OsStr>>(
    key: K,
    values: &Vec<String>,
    options: &ListOptions,
) {
    let separator = get_list_separator(&options);

    if values.is_empty() && options.ignore_empty {
        remove(key)
    } else {
        let value = values.join(&separator);

        set(key, value)
    }
}

pub(crate) fn get_list_with_options<K: AsRef<OsStr>>(
    key: K,
    options: &ListOptions,
) -> Option<Vec<String>> {
    let separator = get_list_separator(&options);

    match env::var(key) {
        Ok(value_string) => {
            if value_string.len() == 0 && !options.ignore_empty {
                Some(vec![])
            } else {
                let values = value_string.split(&separator);
                let mut values_vec = Vec::new();

                for value in values {
                    values_vec.push(value.to_string());
                }

                Some(values_vec)
            }
        }
        _ => None,
    }
}

pub(crate) fn expand_by_prefix(value: &str, prefix: &str) -> String {
    let mut value_string = value.to_string();

    match value_string.find(prefix) {
        Some(_) => {
            for (existing_key, mut existing_value) in env::vars() {
                let mut key_pattern = prefix.to_string();
                key_pattern.push_str(&existing_key);

                let value_length = existing_value.len();
                let pattern_length = key_pattern.len();

                for suffix in " \t\n".chars() {
                    key_pattern.push(suffix);
                    existing_value.push(suffix);

                    value_string = str::replace(&value_string, &key_pattern, &existing_value);

                    key_pattern.remove(pattern_length);
                    existing_value.remove(value_length);
                }

                if value_string.ends_with(&key_pattern) {
                    value_string.truncate(value_string.len() - pattern_length);
                    value_string.push_str(&existing_value);
                }
            }

            value_string
        }
        None => value_string,
    }
}

pub(crate) fn expand_by_wrapper(value: &str, prefix: &str, suffix: &str) -> String {
    let mut value_string = value.to_string();

    match value_string.find(prefix) {
        Some(_) => {
            for (existing_key, existing_value) in env::vars() {
                let mut key_pattern = prefix.to_string();
                key_pattern.push_str(&existing_key);
                key_pattern.push_str(suffix);

                value_string = str::replace(&value_string, &key_pattern, &existing_value);
            }

            value_string
        }
        None => value_string,
    }
}

pub(crate) fn expand(value: &str, expansion_type: ExpansionType) -> String {
    match expansion_type {
        ExpansionType::UnixPrefix => expand_by_prefix(&value, UNIX_ENV_SYMBOL),
        ExpansionType::UnixBrackets => expand_by_wrapper(&value, UNIX_ENV_PREFIX, UNIX_ENV_SUFFIX),
        ExpansionType::Unix => {
            let expanded_value = expand(&value, ExpansionType::UnixBrackets);
            expand(&expanded_value, ExpansionType::UnixPrefix)
        }
        ExpansionType::Windows => expand_by_wrapper(&value, WINDOWS_ENV_SYMBOL, WINDOWS_ENV_SYMBOL),
        ExpansionType::OS => {
            let os_expansion_type = get_os_expansion_type();
            expand(&value, os_expansion_type)
        }
        ExpansionType::All => {
            let expanded_value = expand(&value, ExpansionType::Unix);
            expand(&expanded_value, ExpansionType::Windows)
        }
    }
}

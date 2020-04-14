//! # environment
//!
//! Environment variables direct access functions.
//!

#[cfg(test)]
#[path = "./environment_test.rs"]
mod environment_test;

use crate::expansion;
use crate::types::{ExpandOptions, ExpansionType, ListOptions};
use crate::util;
use std::env;
use std::ffi::OsStr;

static UNIX_ENV_SYMBOL: char = '$';
static UNIX_ENV_PREFIX: &str = "${";
static UNIX_ENV_SUFFIX: char = '}';
static WINDOWS_ENV_SYMBOL_STR: &str = "%";
static WINDOWS_ENV_SYMBOL_CHAR: char = '%';

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

pub(crate) fn set_or_remove<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: &Option<V>) -> bool {
    match value {
        Some(ref value_ref) => {
            set(key, value_ref);
            true
        }
        None => {
            remove(key);
            false
        }
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

pub(crate) fn expand(value: &str, options: Option<ExpandOptions>) -> String {
    if value.len() == 0 {
        return value.to_string();
    }

    let expand_options = match options {
        Some(value) => value,
        None => ExpandOptions::new(),
    };
    let expansion_type = match expand_options.expansion_type {
        Some(value) => value,
        None => ExpansionType::OS,
    };

    match expansion_type {
        ExpansionType::UnixPrefix => {
            expansion::expand_by_prefix(&value, UNIX_ENV_SYMBOL, expand_options.default_to_empty)
        }
        ExpansionType::UnixBrackets => expansion::expand_by_wrapper(
            &value,
            UNIX_ENV_PREFIX,
            UNIX_ENV_SUFFIX,
            expand_options.default_to_empty,
            false,
        ),
        ExpansionType::Unix => {
            let mut cloned_options =
                expand_options.clone_with_expansion_type(ExpansionType::UnixBrackets);
            let expanded_value = expand(&value, Some(cloned_options));
            cloned_options = expand_options.clone_with_expansion_type(ExpansionType::UnixPrefix);
            expand(&expanded_value, Some(cloned_options))
        }
        ExpansionType::Windows => expansion::expand_by_wrapper(
            &value,
            WINDOWS_ENV_SYMBOL_STR,
            WINDOWS_ENV_SYMBOL_CHAR,
            expand_options.default_to_empty,
            false,
        ),
        ExpansionType::OS => {
            let os_expansion_type = expansion::get_os_expansion_type();
            let cloned_options = expand_options.clone_with_expansion_type(os_expansion_type);
            expand(&value, Some(cloned_options))
        }
        ExpansionType::All => {
            let mut cloned_options = expand_options.clone_with_expansion_type(ExpansionType::Unix);
            let expanded_value = expand(&value, Some(cloned_options));
            cloned_options = expand_options.clone_with_expansion_type(ExpansionType::Windows);
            expand(&expanded_value, Some(cloned_options))
        }
        ExpansionType::UnixBracketsWithDefaults => expansion::expand_by_wrapper(
            &value,
            UNIX_ENV_PREFIX,
            UNIX_ENV_SUFFIX,
            expand_options.default_to_empty,
            true,
        ),
    }
}

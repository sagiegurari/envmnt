//! # expansion
//!
//! The expand utility functions.
//!

#[cfg(test)]
#[path = "./expansion_test.rs"]
mod expansion_test;

use crate::types::ExpansionType;
use std::env;

pub(crate) fn get_os_expansion_type() -> ExpansionType {
    if cfg!(windows) {
        ExpansionType::Windows
    } else {
        ExpansionType::Unix
    }
}

fn should_break_key(value: char, reading_key: bool, prefix_only: bool) -> bool {
    if (reading_key && value == ' ')
        || value == '\n'
        || value == '\t'
        || value == '\r'
        || (reading_key && value == '=')
    {
        true
    } else if prefix_only {
        value == '/' || value == '\\' || value == ':'
    } else {
        false
    }
}

pub(crate) fn expand_by_prefix(value: &str, prefix: char, default_to_empty: bool) -> String {
    let mut value_string = String::new();

    let mut found_prefix = false;
    let mut env_key = String::new();
    let mut counter = 0;
    for next_char in value.chars() {
        counter += 1;
        let last_char = counter == value.len();

        if !found_prefix {
            if next_char == prefix {
                found_prefix = true;
                env_key.clear();
            } else {
                value_string.push(next_char);
            }
        } else if last_char || should_break_key(next_char, true, true) {
            if last_char {
                env_key.push(next_char);
            }

            match env::var(&env_key) {
                Ok(env_value) => value_string.push_str(&env_value),
                _ => {
                    if !default_to_empty {
                        value_string.push(prefix);
                        value_string.push_str(&env_key);
                    }
                }
            };

            if !last_char {
                value_string.push(next_char);
            }

            env_key.clear();
            found_prefix = false;
        } else {
            env_key.push(next_char);
        }
    }

    value_string
}

pub(crate) fn expand_by_wrapper(
    value: &str,
    prefix: &str,
    suffix: char,
    default_to_empty: bool,
    search_default: bool,
) -> String {
    let mut value_string = String::new();

    let prefix_length = prefix.len();
    let mut prefix_index = 0;
    let prefix_chars: Vec<char> = prefix.chars().collect();

    let mut found_prefix = false;
    let mut env_key = String::new();
    let mut reading_default = false;
    let mut default_value = String::new();
    for next_char in value.chars() {
        if !found_prefix {
            if next_char == prefix_chars[prefix_index] {
                prefix_index += 1;

                if prefix_index == prefix_length {
                    found_prefix = true;
                    prefix_index = 0;
                    env_key.clear();
                }
            } else {
                if prefix_index > 0 {
                    value_string.push_str(&prefix[..prefix_index]);
                    prefix_index = 0;
                }
                value_string.push(next_char);
            }
        } else if search_default && !reading_default && next_char == ':' {
            reading_default = true;
        } else if next_char == suffix {
            match env::var(&env_key) {
                Ok(env_value) => value_string.push_str(&env_value),
                _ => {
                    if reading_default {
                        value_string.push_str(&default_value);
                    } else if !default_to_empty {
                        value_string.push_str(prefix);
                        value_string.push_str(&env_key);
                        value_string.push(suffix);
                    }
                }
            };

            env_key.clear();
            found_prefix = false;
            if search_default {
                reading_default = false;
                default_value.clear();
            }
        } else if should_break_key(next_char, !reading_default, false) {
            value_string.push_str(prefix);
            value_string.push_str(&env_key);
            if reading_default {
                value_string.push(':');
                value_string.push_str(&default_value);
            }
            value_string.push(next_char);

            env_key.clear();
            found_prefix = false;
            if search_default {
                reading_default = false;
                default_value.clear();
            }
        } else if reading_default {
            default_value.push(next_char);
        } else {
            env_key.push(next_char);
        }
    }

    if !env_key.is_empty() {
        value_string.push_str(prefix);
        value_string.push_str(&env_key);
        if reading_default {
            value_string.push(':');
            value_string.push_str(&default_value);
        }
    }

    value_string
}

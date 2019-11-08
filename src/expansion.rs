//! # expansion
//!
//! The expand utility functions.
//!

#[cfg(test)]
#[path = "./expansion_test.rs"]
mod expansion_test;

use crate::environment;
use crate::types::ExpansionType;

pub(crate) fn get_os_expansion_type() -> ExpansionType {
    if cfg!(windows) {
        ExpansionType::Windows
    } else {
        ExpansionType::Unix
    }
}

pub(crate) fn expand_by_prefix(value: &str, prefix: char, default_to_empty: bool) -> String {
    let mut value_string = String::new();

    let mut found_prefix = false;
    let mut env_key = String::new();
    let mut counter = 0;
    for next_char in value.chars() {
        counter = counter + 1;
        let last_char = counter == value.len();

        if !found_prefix {
            if next_char == prefix {
                found_prefix = true;
                env_key.clear();
            } else {
                value_string.push(next_char);
            }
        } else if last_char
            || next_char == ' '
            || next_char == '\n'
            || next_char == '\t'
            || next_char == '\r'
        {
            if last_char {
                env_key.push(next_char);
            }

            if environment::exists(&env_key) {
                let env_value = environment::get_or(&env_key, "");
                value_string.push_str(&env_value);
            } else if !default_to_empty {
                value_string.push(prefix);
                value_string.push_str(&env_key);
            }

            if !last_char {
                value_string.push(next_char);
            }

            env_key.clear();
            found_prefix = false;
        } else {
            env_key.push(next_char);
        }
    }

    if env_key.len() > 0 {
        value_string.push(prefix);
        value_string.push_str(&env_key);
    }

    value_string
}

pub(crate) fn expand_by_wrapper(
    value: &str,
    prefix: &str,
    suffix: char,
    default_to_empty: bool,
) -> String {
    let mut value_string = String::new();

    let prefix_length = prefix.len();
    let mut prefix_index = 0;
    let prefix_chars: Vec<char> = prefix.chars().collect();

    let mut found_prefix = false;
    let mut env_key = String::new();
    for next_char in value.chars() {
        if !found_prefix {
            if next_char == prefix_chars[prefix_index] {
                prefix_index = prefix_index + 1;

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
        } else if next_char == suffix {
            let default_value = if default_to_empty {
                "".to_string()
            } else {
                env_key.clone()
            };
            let env_value = environment::get_or(&env_key, &default_value);

            value_string.push_str(&env_value);

            env_key.clear();
            found_prefix = false;
        } else {
            env_key.push(next_char);
        }
    }

    if env_key.len() > 0 {
        value_string.push_str(&prefix);
        value_string.push_str(&env_key);
    }

    value_string
}

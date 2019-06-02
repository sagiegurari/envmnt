//! # util
//!
//! Utility functions.
//!

#[cfg(test)]
#[path = "./util_test.rs"]
mod util_test;

pub(crate) fn bool_to_string(value: bool) -> String {
    if value {
        "true".to_string()
    } else {
        "false".to_string()
    }
}

pub(crate) fn string_to_bool(value: &str) -> bool {
    let value_lower_case = value.to_lowercase();

    return value_lower_case.len() > 0
        && value_lower_case != "false"
        && value_lower_case != "no"
        && value_lower_case != "0";
}

//! # numeric
//!
//! Environment variables numeric access functions.
//!

#[cfg(test)]
#[path = "./numeric_test.rs"]
mod numeric_test;

use crate::environment;
use std::env;
use std::ffi::OsStr;

macro_rules! generate_get_numeric {
    ($fn_name:ident, $type:ty) => {
        pub(crate) fn $fn_name<K: AsRef<OsStr>>(key: K, default_value: $type) -> $type {
            match env::var(key) {
                Ok(value) => match value.parse() {
                    Ok(parsed_value) => parsed_value,
                    Err(_) => default_value,
                },
                _ => default_value,
            }
        }
    };
}

generate_get_numeric!(get_u8, u8);
generate_get_numeric!(get_i8, i8);
generate_get_numeric!(get_i16, i16);
generate_get_numeric!(get_u16, u16);
generate_get_numeric!(get_i32, i32);
generate_get_numeric!(get_u32, u32);
generate_get_numeric!(get_i64, i64);
generate_get_numeric!(get_u64, u64);
generate_get_numeric!(get_i128, i128);
generate_get_numeric!(get_u128, u128);
generate_get_numeric!(get_f32, f32);
generate_get_numeric!(get_f64, f64);
generate_get_numeric!(get_isize, isize);
generate_get_numeric!(get_usize, usize);

macro_rules! generate_set_numeric {
    ($fn_name:ident, $type:ty) => {
        pub(crate) fn $fn_name<K: AsRef<OsStr>>(key: K, value: $type) {
            environment::set(key, value.to_string())
        }
    };
}

generate_set_numeric!(set_u8, u8);
generate_set_numeric!(set_i8, i8);
generate_set_numeric!(set_i16, i16);
generate_set_numeric!(set_u16, u16);
generate_set_numeric!(set_i32, i32);
generate_set_numeric!(set_u32, u32);
generate_set_numeric!(set_i64, i64);
generate_set_numeric!(set_u64, u64);
generate_set_numeric!(set_i128, i128);
generate_set_numeric!(set_u128, u128);
generate_set_numeric!(set_f32, f32);
generate_set_numeric!(set_f64, f64);
generate_set_numeric!(set_isize, isize);
generate_set_numeric!(set_usize, usize);

pub(crate) fn increment<K: AsRef<OsStr>>(key: K) -> isize {
    let mut value = get_isize(&key, 0);
    value = value + 1;
    set_isize(&key, value);

    value
}

pub(crate) fn decrement<K: AsRef<OsStr>>(key: K) -> isize {
    let mut value = get_isize(&key, 0);
    value -= 1;
    set_isize(&key, value);

    value
}

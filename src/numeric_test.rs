use super::*;

use std::env;

#[test]
fn get_number_undefined() {
    let output_u8 = get_u8("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u8, 5);
    let output_i8 = get_i8("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i8, 5);
    let output_u16 = get_u16("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u16, 5);
    let output_i16 = get_i16("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i16, 5);
    let output_u32 = get_u32("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u32, 5);
    let output_i32 = get_i32("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i32, 5);
    let output_u64 = get_u64("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u64, 5);
    let output_i64 = get_i64("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i64, 5);
    let output_u128 = get_u128("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u128, 5);
    let output_i128 = get_i128("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i128, 5);
    let output_f32 = get_f32("TEST_GET_NUMBER_UNDEFINED", 5.0);
    assert_eq!(output_f32, 5.0);
    let output_f64 = get_f64("TEST_GET_NUMBER_UNDEFINED", 5.0);
    assert_eq!(output_f64, 5.0);
    let output_usize = get_usize("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_usize, 5);
    let output_isize = get_isize("TEST_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_isize, 5);
}

#[test]
fn get_number_invalid() {
    env::set_var("TEST_GET_NUMBER_INVALID", "abc");
    let output_u8 = get_u8("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u8, 5);
    let output_i8 = get_i8("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i8, 5);
    let output_u16 = get_u16("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u16, 5);
    let output_i16 = get_i16("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i16, 5);
    let output_u32 = get_u32("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u32, 5);
    let output_i32 = get_i32("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i32, 5);
    let output_u64 = get_u64("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u64, 5);
    let output_i64 = get_i64("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i64, 5);
    let output_u128 = get_u128("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u128, 5);
    let output_i128 = get_i128("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i128, 5);
    let output_f32 = get_f32("TEST_GET_NUMBER_INVALID", 5.0);
    assert_eq!(output_f32, 5.0);
    let output_f64 = get_f64("TEST_GET_NUMBER_INVALID", 5.0);
    assert_eq!(output_f64, 5.0);
    let output_usize = get_usize("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_usize, 5);
    let output_isize = get_isize("TEST_GET_NUMBER_INVALID", 5);
    assert_eq!(output_isize, 5);
}

#[test]
fn get_number_valid() {
    env::set_var("TEST_GET_NUMBER_VALID", 15.to_string());
    let output_u8 = get_u8("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_u8, 15);
    let output_i8 = get_i8("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_i8, 15);
    let output_u16 = get_u16("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_u16, 15);
    let output_i16 = get_i16("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_i16, 15);
    let output_u32 = get_u32("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_u32, 15);
    let output_i32 = get_i32("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_i32, 15);
    let output_u64 = get_u64("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_u64, 15);
    let output_i64 = get_i64("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_i64, 15);
    let output_u128 = get_u128("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_u128, 15);
    let output_i128 = get_i128("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_i128, 15);
    let output_f32 = get_f32("TEST_GET_NUMBER_VALID", 5.0);
    assert_eq!(output_f32, 15.0);
    let output_f64 = get_f64("TEST_GET_NUMBER_VALID", 5.0);
    assert_eq!(output_f64, 15.0);
    let output_usize = get_usize("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_usize, 15);
    let output_isize = get_isize("TEST_GET_NUMBER_VALID", 5);
    assert_eq!(output_isize, 15);
}

#[test]
fn set_number_valid() {
    set_u8("TEST_SET_NUMBER_VALID", 15);
    let output_u8 = get_u8("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_u8, 15);
    set_i8("TEST_SET_NUMBER_VALID", 15);
    let output_i8 = get_i8("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_i8, 15);
    set_u16("TEST_SET_NUMBER_VALID", 15);
    let output_u16 = get_u16("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_u16, 15);
    set_i16("TEST_SET_NUMBER_VALID", 15);
    let output_i16 = get_i16("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_i16, 15);
    set_u32("TEST_SET_NUMBER_VALID", 15);
    let output_u32 = get_u32("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_u32, 15);
    set_i32("TEST_SET_NUMBER_VALID", 15);
    let output_i32 = get_i32("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_i32, 15);
    set_u64("TEST_SET_NUMBER_VALID", 15);
    let output_u64 = get_u64("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_u64, 15);
    set_i64("TEST_SET_NUMBER_VALID", 15);
    let output_i64 = get_i64("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_i64, 15);
    set_u128("TEST_SET_NUMBER_VALID", 15);
    let output_u128 = get_u128("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_u128, 15);
    set_i128("TEST_SET_NUMBER_VALID", 15);
    let output_i128 = get_i128("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_i128, 15);
    set_f32("TEST_SET_NUMBER_VALID", 15.0);
    let output_f32 = get_f32("TEST_SET_NUMBER_VALID", 5.0);
    assert_eq!(output_f32, 15.0);
    set_f64("TEST_SET_NUMBER_VALID", 15.0);
    let output_f64 = get_f64("TEST_SET_NUMBER_VALID", 5.0);
    assert_eq!(output_f64, 15.0);
    set_usize("TEST_SET_NUMBER_VALID", 15);
    let output_usize = get_usize("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_usize, 15);
    set_isize("TEST_SET_NUMBER_VALID", 15);
    let output_isize = get_isize("TEST_SET_NUMBER_VALID", 5);
    assert_eq!(output_isize, 15);
}

#[test]
fn increment_undefined() {
    let value = increment("TEST_INCREMENT_UNDEFINED");
    assert_eq!(value, 1);
}

#[test]
fn increment_invalid() {
    environment::set("TEST_INCREMENT_INVALID", "abc");
    let value = increment("TEST_INCREMENT_INVALID");
    assert_eq!(value, 1);
}

#[test]
fn increment_valid() {
    set_u8("TEST_INCREMENT_VALID", 50);
    let value = increment("TEST_INCREMENT_VALID");
    assert_eq!(value, 51);
}

#[test]
fn decrement_undefined() {
    let value = decrement("TEST_DECREMENT_UNDEFINED");
    assert_eq!(value, -1);
}

#[test]
fn decrement_invalid() {
    environment::set("TEST_DECREMENT_INVALID", "abc");
    let value = decrement("TEST_DECREMENT_INVALID");
    assert_eq!(value, -1);
}

#[test]
fn decrement_valid() {
    set_u8("TEST_DECREMENT_VALID", 50);
    let value = decrement("TEST_DECREMENT_VALID");
    assert_eq!(value, 49);
}

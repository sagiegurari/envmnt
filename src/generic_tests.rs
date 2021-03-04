use super::*;

use std::env;
use std::ffi::OsString;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::SocketAddrV6;
use std::num::NonZeroI128;
use std::num::NonZeroI16;
use std::num::NonZeroI32;
use std::num::NonZeroI64;
use std::num::NonZeroI8;
use std::num::NonZeroIsize;
use std::num::NonZeroU128;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::num::NonZeroU64;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;
use std::path::PathBuf;

#[test]
fn get_parse_undefined() {
    let output_socketaddr: Result<SocketAddr, EnvmntError> = get_parse("TEST_GET_SOCKETADDR_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_socketaddr {true} else {false});

    let output_bool: Result<bool, EnvmntError> = get_parse("TEST_GET_BOOL_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_bool {true} else {false});

    let output_char: Result<char, EnvmntError> = get_parse("TEST_GET_CHAR_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_char {true} else {false});

    let output_f32: Result<f32, EnvmntError> = get_parse("TEST_GET_F32_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_f32 {true} else {false});

    let output_f64: Result<f64, EnvmntError> = get_parse("TEST_GET_F64_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_f64 {true} else {false});

    let output_i8: Result<i8, EnvmntError> = get_parse("TEST_GET_I8_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_i8 {true} else {false});

    let output_i16: Result<i16, EnvmntError> = get_parse("TEST_GET_I16_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_i16 {true} else {false});

    let output_i32: Result<i32, EnvmntError> = get_parse("TEST_GET_I32_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_i32 {true} else {false});

    let output_i64: Result<i64, EnvmntError> = get_parse("TEST_GET_I64_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_i64 {true} else {false});

    let output_i128: Result<i128, EnvmntError> = get_parse("TEST_GET_I128_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_i128 {true} else {false});

    let output_isize: Result<isize, EnvmntError> = get_parse("TEST_GET_ISIZE_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_isize {true} else {false});

    let output_u8: Result<u8, EnvmntError> = get_parse("TEST_GET_U8_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_u8 {true} else {false});

    let output_u16: Result<u16, EnvmntError> = get_parse("TEST_GET_U16_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_u16 {true} else {false});

    let output_u32: Result<u32, EnvmntError> = get_parse("TEST_GET_U32_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_u32 {true} else {false});

    let output_u64: Result<u64, EnvmntError> = get_parse("TEST_GET_U64_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_u64 {true} else {false});

    let output_u128: Result<u128, EnvmntError> = get_parse("TEST_GET_U128_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_u128 {true} else {false});

    let output_usize: Result<usize, EnvmntError> = get_parse("TEST_GET_USIZE_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_usize {true} else {false});

    let output_osstring: Result<OsString, EnvmntError> = get_parse("TEST_GET_OSSTRING_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_osstring {true} else {false});

    let output_ipv4addr: Result<Ipv4Addr, EnvmntError> = get_parse("TEST_GET_IPV4ADDR_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_ipv4addr {true} else {false});

    let output_ipv6addr: Result<Ipv6Addr, EnvmntError> = get_parse("TEST_GET_IPV6ADDR_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_ipv6addr {true} else {false});

    let output_socketaddrv4: Result<SocketAddrV4, EnvmntError> = get_parse("TEST_GET_SOCKETADDRV4_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_socketaddrv4 {true} else {false});

    let output_socketaddrv6: Result<SocketAddrV6, EnvmntError> = get_parse("TEST_GET_SOCKETADDRV6_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_socketaddrv6 {true} else {false});

    let output_nonzeroi8: Result<NonZeroI8, EnvmntError> = get_parse("TEST_GET_NONZEROI8_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzeroi8 {true} else {false});

    let output_nonzeroi16: Result<NonZeroI16, EnvmntError> = get_parse("TEST_GET_NONZEROI16_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzeroi16 {true} else {false});

    let output_nonzeroi32: Result<NonZeroI32, EnvmntError> = get_parse("TEST_GET_NONZEROI32_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzeroi32 {true} else {false});

    let output_nonzeroi64: Result<NonZeroI64, EnvmntError> = get_parse("TEST_GET_NONZEROI64_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzeroi64 {true} else {false});

    let output_nonzeroi128: Result<NonZeroI128, EnvmntError> = get_parse("TEST_GET_NONZEROI128_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzeroi128 {true} else {false});

    let output_nonzeroisize: Result<NonZeroIsize, EnvmntError> = get_parse("TEST_GET_NONZEROISIZE_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzeroisize {true} else {false});

    let output_nonzerou8: Result<NonZeroU8, EnvmntError> = get_parse("TEST_GET_NONZEROU8_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzerou8 {true} else {false});

    let output_nonzerou16: Result<NonZeroU16, EnvmntError> = get_parse("TEST_GET_NONZEROU16_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzerou16 {true} else {false});

    let output_nonzerou32: Result<NonZeroU32, EnvmntError> = get_parse("TEST_GET_NONZEROU32_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzerou32 {true} else {false});

    let output_nonzerou64: Result<NonZeroU64, EnvmntError> = get_parse("TEST_GET_NONZEROU64_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzerou64 {true} else {false});

    let output_nonzerou128: Result<NonZeroU128, EnvmntError> = get_parse("TEST_GET_NONZEROU128_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzerou128 {true} else {false});

    let output_nonzerousize: Result<NonZeroUsize, EnvmntError> = get_parse("TEST_GET_NONZEROUSIZE_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_nonzerousize {true} else {false});

    let output_pathbuf: Result<PathBuf, EnvmntError> = get_parse("TEST_GET_PATHBUF_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_pathbuf {true} else {false});

    let output_string: Result<String, EnvmntError> = get_parse("TEST_GET_STRING_UNDEFINED");
    assert!(if let Err(EnvmntError {kind: ErrorKind::Missing(_)}) = output_string {true} else {false});
}

#[test]
fn get_parse_default_undefined() {
    let output_bool: Result<bool, EnvmntError> = get_parse_or("TEST_GET_BOOL_UNDEFINED", true);
    assert!(if let Ok(true) = output_bool {true} else {false});

    let output_char: Result<char, EnvmntError> = get_parse_or("TEST_GET_CHAR_UNDEFINED", 'A');
    assert!(if let Ok('A') = output_char {true} else {false});

    let output_i8: Result<i8, EnvmntError> = get_parse_or("TEST_GET_I8_UNDEFINED", -123);
    assert!(if let Ok(-123) = output_i8 {true} else {false});

    let output_i16: Result<i16, EnvmntError> = get_parse_or("TEST_GET_I16_UNDEFINED", -123);
    assert!(if let Ok(-123) = output_i16 {true} else {false});

    let output_i32: Result<i32, EnvmntError> = get_parse_or("TEST_GET_I32_UNDEFINED", -123);
    assert!(if let Ok(-123) = output_i32 {true} else {false});

    let output_i64: Result<i64, EnvmntError> = get_parse_or("TEST_GET_I64_UNDEFINED", -123);
    assert!(if let Ok(-123) = output_i64 {true} else {false});

    let output_i128: Result<i128, EnvmntError> = get_parse_or("TEST_GET_I128_UNDEFINED", -123);
    assert!(if let Ok(-123) = output_i128 {true} else {false});

    let output_isize: Result<isize, EnvmntError> = get_parse_or("TEST_GET_ISIZE_UNDEFINED", -123);
    assert!(if let Ok(-123) = output_isize {true} else {false});

    let output_u8: Result<u8, EnvmntError> = get_parse_or("TEST_GET_U8_UNDEFINED", 123);
    assert!(if let Ok(123) = output_u8 {true} else {false});

    let output_u16: Result<u16, EnvmntError> = get_parse_or("TEST_GET_U16_UNDEFINED", 123);
    assert!(if let Ok(123) = output_u16 {true} else {false});

    let output_u32: Result<u32, EnvmntError> = get_parse_or("TEST_GET_U32_UNDEFINED", 123);
    assert!(if let Ok(123) = output_u32 {true} else {false});

    let output_u64: Result<u64, EnvmntError> = get_parse_or("TEST_GET_U64_UNDEFINED", 123);
    assert!(if let Ok(123) = output_u64 {true} else {false});

    let output_u128: Result<u128, EnvmntError> = get_parse_or("TEST_GET_U128_UNDEFINED", 123);
    assert!(if let Ok(123) = output_u128 {true} else {false});

    let output_usize: Result<usize, EnvmntError> = get_parse_or("TEST_GET_USIZE_UNDEFINED", 123);
    assert!(if let Ok(123) = output_usize {true} else {false});
}

#[test]
fn get_parse_invalid() {
    env::set_var("TEST_GET_BOOL_INVALID", "abc");
    env::set_var("TEST_GET_CHAR_INVALID", "abc");
    env::set_var("TEST_GET_NUMBER_INVALID", "abc");

    let output_bool: Result<bool, EnvmntError> = get_parse("TEST_GET_BOOL_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_bool {true} else {false});

    let output_char: Result<char, EnvmntError> = get_parse("TEST_GET_CHAR_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_char {true} else {false});

    let output_i8: Result<i8, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_i8 {true} else {false});

    let output_i16: Result<i16, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_i16 {true} else {false});

    let output_i32: Result<i32, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_i32 {true} else {false});

    let output_i64: Result<i64, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_i64 {true} else {false});

    let output_i128: Result<i128, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_i128 {true} else {false});

    let output_isize: Result<isize, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_isize {true} else {false});

    let output_u8: Result<u8, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_u8 {true} else {false});

    let output_u16: Result<u16, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_u16 {true} else {false});

    let output_u32: Result<u32, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_u32 {true} else {false});

    let output_u64: Result<u64, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_u64 {true} else {false});

    let output_u128: Result<u128, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_u128 {true} else {false});

    let output_usize: Result<usize, EnvmntError> = get_parse("TEST_GET_NUMBER_INVALID");
    assert!(if let Err(EnvmntError {kind: ErrorKind::InvalidType(_)}) = output_usize {true} else {false});
}

#[test]
fn get_parse_valid() {
    env::set_var("TEST_GET_BOOL_VALID", "true");
    env::set_var("TEST_GET_CHAR_VALID", "A");
    env::set_var("TEST_GET_INUMBER_VALID", "-123");
    env::set_var("TEST_GET_UNUMBER_VALID", "123");

    let output_bool: Result<bool, EnvmntError> = get_parse("TEST_GET_BOOL_VALID");
    assert!(if let Ok(true) = output_bool {true} else {false});

    let output_char: Result<char, EnvmntError> = get_parse("TEST_GET_CHAR_VALID");
    assert!(if let Ok('A') = output_char {true} else {false});

    let output_i8: Result<i8, EnvmntError> = get_parse("TEST_GET_INUMBER_VALID");
    assert!(if let Ok(-123) = output_i8 {true} else {false});

    let output_i16: Result<i16, EnvmntError> = get_parse("TEST_GET_INUMBER_VALID");
    assert!(if let Ok(-123) = output_i16 {true} else {false});

    let output_i32: Result<i32, EnvmntError> = get_parse("TEST_GET_INUMBER_VALID");
    assert!(if let Ok(-123) = output_i32 {true} else {false});

    let output_i64: Result<i64, EnvmntError> = get_parse("TEST_GET_INUMBER_VALID");
    assert!(if let Ok(-123) = output_i64 {true} else {false});

    let output_i128: Result<i128, EnvmntError> = get_parse("TEST_GET_INUMBER_VALID");
    assert!(if let Ok(-123) = output_i128 {true} else {false});

    let output_isize: Result<isize, EnvmntError> = get_parse("TEST_GET_INUMBER_VALID");
    assert!(if let Ok(-123) = output_isize {true} else {false});

    let output_u8: Result<u8, EnvmntError> = get_parse("TEST_GET_UNUMBER_VALID");
    assert!(if let Ok(123) = output_u8 {true} else {false});

    let output_u16: Result<u16, EnvmntError> = get_parse("TEST_GET_UNUMBER_VALID");
    assert!(if let Ok(123) = output_u16 {true} else {false});

    let output_u32: Result<u32, EnvmntError> = get_parse("TEST_GET_UNUMBER_VALID");
    assert!(if let Ok(123) = output_u32 {true} else {false});

    let output_u64: Result<u64, EnvmntError> = get_parse("TEST_GET_UNUMBER_VALID");
    assert!(if let Ok(123) = output_u64 {true} else {false});

    let output_u128: Result<u128, EnvmntError> = get_parse("TEST_GET_UNUMBER_VALID");
    assert!(if let Ok(123) = output_u128 {true} else {false});

    let output_usize: Result<usize, EnvmntError> = get_parse("TEST_GET_UNUMBER_VALID");
    assert!(if let Ok(123) = output_usize {true} else {false});
}

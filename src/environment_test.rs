use super::*;

use std::env;

#[test]
fn exists_false() {
    let output = exists("TEST_EXISTS_FALSE");
    assert!(!output);
}

#[test]
fn exists_true() {
    env::set_var("TEST_EXISTS_TRUE", "EXISTS");
    let output = exists("TEST_EXISTS_TRUE");
    assert!(output);
}

#[test]
fn remove_exists() {
    env::set_var("TEST_REMOVE_EXISTS", "EXISTS");
    remove("TEST_REMOVE_EXISTS");
    let output = exists("TEST_REMOVE_EXISTS");
    assert!(!output);
}

#[test]
fn remove_not_exists() {
    remove("TEST_REMOVE_NOT_EXISTS");
}

#[test]
fn get_remove_not_exists() {
    let output = get_remove("TEST_GET_REMOVE_NOT_EXISTS");
    assert!(output.is_none());
}

#[test]
fn get_remove_exists() {
    env::set_var("TEST_GET_REMOVE_EXISTS", "OLD");
    let output = get_remove("TEST_GET_REMOVE_EXISTS");
    assert_eq!(output.unwrap(), "OLD".to_string());
}

#[test]
fn get_or_exists() {
    env::set_var("TEST_GET_OR_EXISTS", "EXISTS");
    let output = get_or("TEST_GET_OR_EXISTS", "bad");
    assert_eq!(output, "EXISTS".to_string());
}

#[test]
fn get_or_not_exists() {
    let output = get_or("TEST_GET_OR_NOT_EXISTS", "good");
    assert_eq!(output, "good".to_string());
}

#[test]
fn get_or_empty() {
    env::set_var("TEST_GET_OR_EMPTY", "");
    let output = get_or("TEST_GET_OR_EMPTY", "bad");
    assert_eq!(output, "".to_string());
}

#[test]
fn get_or_panic_exists() {
    env::set_var("TEST_GET_OR_PANIC_EXISTS", "EXISTS");
    let output = get_or_panic("TEST_GET_OR_PANIC_EXISTS");
    assert_eq!(output, "EXISTS".to_string());
}

#[test]
#[should_panic]
fn get_or_panic_not_exists() {
    get_or_panic("TEST_GET_OR_PANIC_NOT_EXISTS");
}

#[test]
fn get_or_panic_empty() {
    env::set_var("TEST_GET_OR_PANIC_EMPTY", "");
    let output = get_or_panic("TEST_GET_OR_PANIC_EMPTY");
    assert_eq!(output, "".to_string());
}

#[test]
fn is_or_false() {
    env::set_var("TEST_IS_OR_BOOL_FALSE", "false");
    let output = is_or("TEST_IS_OR_BOOL_FALSE", true);
    assert!(!output);
}

#[test]
fn is_or_false_uppercase() {
    env::set_var("TEST_IS_OR_BOOL_FALSE_UPPER", "FALSE");
    let output = is_or("TEST_IS_OR_BOOL_FALSE_UPPER", true);
    assert!(!output);
}

#[test]
fn is_or_no() {
    env::set_var("TEST_IS_OR_BOOL_NO", "no");
    let output = is_or("TEST_IS_OR_BOOL_NO", true);
    assert!(!output);
}

#[test]
fn is_or_no_uppercase() {
    env::set_var("TEST_IS_OR_BOOL_NO_UPPER", "NO");
    let output = is_or("TEST_IS_OR_BOOL_NO_UPPER", true);
    assert!(!output);
}

#[test]
fn is_or_zero() {
    env::set_var("TEST_IS_OR_BOOL_ZERO", "0");
    let output = is_or("TEST_IS_OR_BOOL_ZERO", true);
    assert!(!output);
}

#[test]
fn is_or_empty() {
    env::set_var("TEST_IS_OR_BOOL_EMPTY", "");
    let output = is_or("TEST_IS_OR_BOOL_EMPTY", false);
    assert!(!output);
}

#[test]
fn is_or_else() {
    env::set_var("TEST_IS_OR_BOOL_ELSE", "true");
    let output = is_or("TEST_IS_OR_BOOL_ELSE", false);
    assert!(output);
}

#[test]
fn is_or_default_true() {
    let output = is_or("TEST_IS_OR_BOOL_NO_EXISTS_TRUE", true);
    assert!(output);
}

#[test]
fn is_or_default_false() {
    let output = is_or("TEST_IS_OR_BOOL_NO_EXISTS_FALSE", false);
    assert!(!output);
}

#[test]
fn set_value() {
    set("TEST_SET_VALUE", "SIMPLE");
    assert_eq!(env::var("TEST_SET_VALUE").unwrap(), "SIMPLE".to_string());
}

#[test]
fn set_bool_false() {
    set_bool("TEST_SET_BOOL_FALSE", false);
    assert_eq!(
        env::var("TEST_SET_BOOL_FALSE").unwrap(),
        "false".to_string()
    );
}

#[test]
fn set_bool_true() {
    set_bool("TEST_SET_BOOL_TRUE", true);
    assert_eq!(env::var("TEST_SET_BOOL_TRUE").unwrap(), "true".to_string());
}

#[test]
fn get_set_exists() {
    env::set_var("TEST_GET_SET_EXISTS", "OLD");
    let output = get_set("TEST_GET_SET_EXISTS", "NEW");
    assert_eq!(output.unwrap(), "OLD".to_string());
    assert_eq!(env::var("TEST_GET_SET_EXISTS").unwrap(), "NEW".to_string());
}

#[test]
fn get_set_not_exists() {
    let output = get_set("TEST_GET_SET_NOT_EXISTS", "NEW");
    assert!(output.is_none());
    assert_eq!(
        env::var("TEST_GET_SET_NOT_EXISTS").unwrap(),
        "NEW".to_string()
    );
}

#[test]
fn is_equal_not_exists() {
    let output = is_equal("TEST_IS_EQUAL_NOT_EXISTS", "VALUE");
    assert!(!output);
}

#[test]
fn is_equal_same() {
    env::set_var("TEST_IS_EQUAL_SAME", "VALUE");
    let output = is_equal("TEST_IS_EQUAL_SAME", "VALUE");
    assert!(output);
}

#[test]
fn is_equal_not_same() {
    env::set_var("TEST_IS_EQUAL_NOT_SAME", "1");
    let output = is_equal("TEST_IS_EQUAL_NOT_SAME", "2");
    assert!(!output);
}

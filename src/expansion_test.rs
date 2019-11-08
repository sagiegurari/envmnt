use super::*;

use crate::environment;

#[test]
fn expand_by_prefix_found() {
    environment::set("TEST_EXPAND_BY_PREFIX_FOUND1", "test1");
    environment::set("TEST_EXPAND_BY_PREFIX_FOUND2", "test2");
    environment::set("TEST_EXPAND_BY_PREFIX_FOUND3", "test3");
    environment::set("TEST_EXPAND_BY_PREFIX_FOUND4", "test4");

    let output = expand_by_prefix(
        r#"
value1:$TEST_EXPAND_BY_PREFIX_FOUND1
value2:$TEST_EXPAND_BY_PREFIX_FOUND2
value3:$TEST_EXPAND_BY_PREFIX_FOUND3
value4:$TEST_EXPAND_BY_PREFIX_FOUND4

value1:$TEST_EXPAND_BY_PREFIX_FOUND1
value2:$TEST_EXPAND_BY_PREFIX_FOUND2
value3:$TEST_EXPAND_BY_PREFIX_FOUND3
value4:$TEST_EXPAND_BY_PREFIX_FOUND4
    "#,
        '$',
        true,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:test4

value1:test1
value2:test2
value3:test3
value4:test4
    "#,
        output
    );
}

#[test]
fn expand_by_prefix_partial_found_empty_default() {
    environment::set(
        "TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT1",
        "test1",
    );
    environment::set(
        "TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT2",
        "test2",
    );
    environment::set(
        "TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT3",
        "test3",
    );

    let output = expand_by_prefix(
        r#"
value1:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT1
value2:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT2
value3:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT3
value4:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT4

value1:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT1
value2:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT2
value3:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT3
value4:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_EMPTY_DEFAULT4
    "#,
        '$',
        true,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:

value1:test1
value2:test2
value3:test3
value4:
    "#,
        output
    );
}

#[test]
fn expand_by_prefix_partial_found_key_default() {
    environment::set("TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT1", "test1");
    environment::set("TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT2", "test2");
    environment::set("TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT3", "test3");

    let output = expand_by_prefix(
        r#"
value1:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT1
value2:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT2
value3:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT3
value4:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT4

value1:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT1
value2:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT2
value3:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT3
value4:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT4
    "#,
        '$',
        false,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT4

value1:test1
value2:test2
value3:test3
value4:$TEST_EXPAND_BY_PREFIX_PARTIAL_FOUND_KEY_DEFAULT4
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_found() {
    environment::set("TEST_EXPAND_BY_WRAPPER_FOUND1", "test1");
    environment::set("TEST_EXPAND_BY_WRAPPER_FOUND2", "test2");
    environment::set("TEST_EXPAND_BY_WRAPPER_FOUND3", "test3");
    environment::set("TEST_EXPAND_BY_WRAPPER_FOUND4", "test4");

    let output = expand_by_wrapper(
        r#"
value1:${TEST_EXPAND_BY_WRAPPER_FOUND1}
value2:${TEST_EXPAND_BY_WRAPPER_FOUND2}
value3:${TEST_EXPAND_BY_WRAPPER_FOUND3}
value4:${TEST_EXPAND_BY_WRAPPER_FOUND4}

value1:${TEST_EXPAND_BY_WRAPPER_FOUND1}
value2:${TEST_EXPAND_BY_WRAPPER_FOUND2}
value3:${TEST_EXPAND_BY_WRAPPER_FOUND3}
value4:${TEST_EXPAND_BY_WRAPPER_FOUND4}
    "#,
        "${",
        '}',
        true,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:test4

value1:test1
value2:test2
value3:test3
value4:test4
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_partial_found_empty_default() {
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT1",
        "test1",
    );
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT2",
        "test2",
    );
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT3",
        "test3",
    );

    let output = expand_by_wrapper(
        r#"
value1:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT4}

value1:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT4}
    "#,
        "${",
        '}',
        true,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:

value1:test1
value2:test2
value3:test3
value4:
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_partial_found_key_default() {
    environment::set("TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT1", "test1");
    environment::set("TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT2", "test2");
    environment::set("TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT3", "test3");

    let output = expand_by_wrapper(
        r#"
value1:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT4}

value1:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT4}
    "#,
        "${",
        '}',
        false,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT4}

value1:test1
value2:test2
value3:test3
value4:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_KEY_DEFAULT4}
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_no_suffix() {
    environment::set("TEST_EXPAND_BY_WRAPPER_NO_SUFFIX1", "test1");
    environment::set("TEST_EXPAND_BY_WRAPPER_NO_SUFFIX2", "test2");

    let output = expand_by_wrapper(
        r#"
value1:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX1}
value2:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX2}
value3:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX3

value1:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX1}
value2:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX2}
value3:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX3
    "#,
        "${",
        '}',
        false,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX3

value1:test1
value2:test2
value3:${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX3
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_no_suffix_single() {
    let output = expand_by_wrapper(
        "${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX_SINGLE",
        "${",
        '}',
        false,
    );

    assert_eq!("${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX_SINGLE", output);
}

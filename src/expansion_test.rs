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
fn expand_by_prefix_breaks() {
    environment::set("TEST_EXPAND_BY_PREFIX_BREAKS1", "test1");
    environment::set("TEST_EXPAND_BY_PREFIX_BREAKS2", "test2");
    environment::set("TEST_EXPAND_BY_PREFIX_BREAKS3", "test3");
    environment::set("TEST_EXPAND_BY_PREFIX_BREAKS4", "test4");
    environment::set("TEST_EXPAND_BY_PREFIX_BREAKS5", "test5");

    let output = expand_by_prefix(
        r#"
value1:$TEST_EXPAND_BY_PREFIX_BREAKS1 A
value2:$TEST_EXPAND_BY_PREFIX_BREAKS2	A
value3:$TEST_EXPAND_BY_PREFIX_BREAKS3=
value4:$TEST_EXPAND_BY_PREFIX_BREAKS4/
value5:$TEST_EXPAND_BY_PREFIX_BREAKS5\
    "#,
        '$',
        true,
    );

    assert_eq!(
        r#"
value1:test1 A
value2:test2	A
value3:test3=
value4:test4/
value5:test5\
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
        false,
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
value5:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT5:default}

value1:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT4}
value5:${TEST_EXPAND_BY_WRAPPER_PARTIAL_FOUND_EMPTY_DEFAULT5:default}
    "#,
        "${",
        '}',
        true,
        false,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:
value5:

value1:test1
value2:test2
value3:test3
value4:
value5:
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
        false,
    );

    assert_eq!("${TEST_EXPAND_BY_WRAPPER_NO_SUFFIX_SINGLE", output);
}

#[test]
fn expand_by_wrapper_embedded_partial_found_key_default() {
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT1",
        "test1",
    );
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT2",
        "test2",
    );
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT3",
        "test3",
    );

    let output = expand_by_wrapper(
        r#"
value1:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT4}
value5:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT5:default 5}

value1:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT4}
value5:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT5:default 5 again}
    "#,
        "${",
        '}',
        false,
        true,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT4}
value5:default 5

value1:test1
value2:test2
value3:test3
value4:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_KEY_DEFAULT4}
value5:default 5 again
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_embedded_no_suffix() {
    environment::set("TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX1", "test1");
    environment::set("TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX2", "test2");

    let output = expand_by_wrapper(
        r#"
value1:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX1}
value2:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX2}
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX3:default 3

value1:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX1}
value2:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX2}
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX3:
    "#,
        "${",
        '}',
        false,
        true,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX3:default 3

value1:test1
value2:test2
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX3:
    "#,
        output
    );
}

#[test]
fn expand_by_wrapper_embedded_no_suffix_single() {
    let output = expand_by_wrapper(
        "${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX_SINGLE:default value",
        "${",
        '}',
        false,
        true,
    );

    assert_eq!(
        "${TEST_EXPAND_BY_WRAPPER_EMBEDDED_NO_SUFFIX_SINGLE:default value",
        output
    );
}

#[test]
fn expand_by_wrapper_embedded_partial_found_empty_default() {
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT1",
        "test1",
    );
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT2",
        "test2",
    );
    environment::set(
        "TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT3",
        "test3",
    );

    let output = expand_by_wrapper(
        r#"
value1:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT4}
value5:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT5:default 5}

value1:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT1}
value2:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT2}
value3:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT3}
value4:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT4}
value5:${TEST_EXPAND_BY_WRAPPER_EMBEDDED_PARTIAL_FOUND_EMPTY_DEFAULT5:default 5 again}
    "#,
        "${",
        '}',
        true,
        true,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:
value5:default 5

value1:test1
value2:test2
value3:test3
value4:
value5:default 5 again
    "#,
        output
    );
}

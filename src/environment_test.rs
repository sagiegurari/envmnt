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
fn get_any_exists() {
    env::set_var("TEST_GET_ANY_EXISTS2", "EXISTS2");
    let output = get_any(
        &vec!["TEST_GET_ANY_EXISTS1", "TEST_GET_ANY_EXISTS2"],
        "not found",
    );
    assert_eq!(output, "EXISTS2".to_string());
}

#[test]
fn get_any_not_exists() {
    let output = get_any(
        &vec!["TEST_GET_ANY_NOT_EXISTS1", "TEST_GET_ANY_NOT_EXISTS2"],
        "not found",
    );
    assert_eq!(output, "not found".to_string());
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
fn is_or_true() {
    env::set_var("TEST_IS_OR_BOOL_TRUE", "true");
    let output = is_or("TEST_IS_OR_BOOL_TRUE", false);
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
fn is_false() {
    env::set_var("TEST_IS_BOOL_FALSE", "false");
    let output = is("TEST_IS_BOOL_FALSE");
    assert!(!output);
}

#[test]
fn is_empty() {
    env::set_var("TEST_IS_BOOL_EMPTY", "");
    let output = is("TEST_IS_BOOL_EMPTY");
    assert!(!output);
}

#[test]
fn is_true() {
    env::set_var("TEST_IS_BOOL_TRUE", "true");
    let output = is("TEST_IS_BOOL_TRUE");
    assert!(output);
}

#[test]
fn is_undefined() {
    let output = is("TEST_IS_BOOL_UNDEFINED");
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
fn set_optional_none() {
    let output = set_optional::<&str, &str>("TEST_SET_OPTIONAL_NONE", &None);

    assert!(!output);
}

#[test]
fn set_optional_some() {
    let output = set_optional("TEST_SET_OPTIONAL_SOME", &Some("OPTIONAL VALUE"));

    assert!(output);
    assert_eq!(
        env::var("TEST_SET_OPTIONAL_SOME").unwrap(),
        "OPTIONAL VALUE".to_string()
    );
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
fn vars_valid() {
    let output = vars();
    assert!(output.len() > 0);

    for (key, value) in output {
        if !key.starts_with("TEST") {
            assert!(is_equal(&key, &value));
        }
    }
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

#[test]
fn contains_not_exists() {
    let output = contains("TEST_CONTAINS_NOT_EXISTS", "VALUE");
    assert!(!output);
}

#[test]
fn contains_same() {
    env::set_var("TEST_CONTAINS_SAME", "VALUE");
    let output = contains("TEST_CONTAINS_SAME", "VAL");
    assert!(output);
}

#[test]
fn contains_not_same() {
    env::set_var("TEST_CONTAINS_NOT_SAME", "VALUE");
    let output = contains("TEST_CONTAINS_NOT_SAME", "val");
    assert!(!output);
}

#[test]
fn contains_ignore_case_not_exists() {
    let output = contains_ignore_case("TEST_CONTAINS_IGNORE_CASE_NOT_EXISTS", "VALUE");
    assert!(!output);
}

#[test]
fn contains_ignore_case_same() {
    env::set_var("TEST_CONTAINS_IGNORE_CASE_SAME", "VALUE");
    let output = contains_ignore_case("TEST_CONTAINS_IGNORE_CASE_SAME", "VAL");
    assert!(output);
}

#[test]
fn contains_ignore_case_same_value_different_case() {
    env::set_var("TEST_CONTAINS_IGNORE_CASE_SAME_DIFFERENT_CASE", "VALUE");
    let output = contains_ignore_case("TEST_CONTAINS_IGNORE_CASE_SAME_DIFFERENT_CASE", "val");
    assert!(output);
}

#[test]
fn contains_ignore_case_not_same() {
    env::set_var("TEST_CONTAINS_IGNORE_CASE_NOT_SAME", "1");
    let output = contains_ignore_case("TEST_CONTAINS_IGNORE_CASE_NOT_SAME", "2");
    assert!(!output);
}

#[test]
fn set_list_empty() {
    set_list("TEST_SET_LIST_EMPTY", &vec![]);

    let output = get_or_panic("TEST_SET_LIST_EMPTY");
    assert_eq!(output, "");
}

#[test]
fn set_list_single_empty() {
    set_list("TEST_SET_LIST_SINGLE_EMPTY", &vec!["".to_string()]);

    let output = is_equal("TEST_SET_LIST_SINGLE_EMPTY", "");
    assert!(output);
}

#[test]
fn set_list_single() {
    set_list("TEST_SET_LIST_SINGLE", &vec!["1".to_string()]);

    let output = is_equal("TEST_SET_LIST_SINGLE", "1");
    assert!(output);
}

#[test]
fn set_list_multiple() {
    set_list(
        "TEST_SET_LIST_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let output = is_equal("TEST_SET_LIST_MULTIPLE", "1;2;3");
    assert!(output);
}

#[test]
fn get_list_none() {
    let output = get_list("TEST_GET_LIST_NONE").is_none();
    assert!(output);
}

#[test]
fn get_list_empty() {
    set_list("TEST_GET_LIST_EMPTY", &vec![]);

    let output = get_list("TEST_GET_LIST_EMPTY").unwrap();
    assert!(output.is_empty());
}

#[test]
fn get_list_single() {
    set_list("TEST_GET_LIST_SINGLE", &vec!["1".to_string()]);

    let output = get_list("TEST_GET_LIST_SINGLE").unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output, vec!["1".to_string()]);
}

#[test]
fn get_list_multiple() {
    set_list(
        "TEST_GET_LIST_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let output = get_list("TEST_GET_LIST_MULTIPLE").unwrap();
    assert_eq!(output.len(), 3);
    assert_eq!(
        output,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
}

#[test]
fn set_list_with_options_empty() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options("TEST_SET_LIST_WITH_OPTIONS_EMPTY", &vec![], &options);

    let output = get_or_panic("TEST_SET_LIST_WITH_OPTIONS_EMPTY");
    assert_eq!(output, "");
}

#[test]
fn set_list_with_options_single_empty() {
    let options = ListOptions::new();

    set_list_with_options(
        "TEST_SET_LIST_WITH_OPTIONS_SINGLE_EMPTY",
        &vec!["".to_string()],
        &options,
    );

    let output = is_equal("TEST_SET_LIST_WITH_OPTIONS_SINGLE_EMPTY", "");
    assert!(output);
}

#[test]
fn set_list_with_options_single() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options(
        "TEST_SET_LIST_WITH_OPTIONS_SINGLE",
        &vec!["1".to_string()],
        &options,
    );

    let output = is_equal("TEST_SET_LIST_WITH_OPTIONS_SINGLE", "1");
    assert!(output);
}

#[test]
fn set_list_with_options_multiple() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options(
        "TEST_SET_LIST_WITH_OPTIONS_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        &options,
    );

    let output = is_equal("TEST_SET_LIST_WITH_OPTIONS_MULTIPLE", "1,2,3");
    assert!(output);
}

#[test]
fn get_list_with_options_none() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    let output = get_list_with_options("TEST_GET_LIST_WITH_OPTIONS_NONE", &options).is_none();
    assert!(output);
}

#[test]
fn get_list_with_options_empty() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options("TEST_GET_LIST_WITH_OPTIONS_EMPTY", &vec![], &options);

    let output = get_list_with_options("TEST_GET_LIST_WITH_OPTIONS_EMPTY", &options).unwrap();
    assert!(output.is_empty());
}

#[test]
fn get_list_with_options_single_empty_ignore() {
    let mut options = ListOptions::new();
    options.ignore_empty = true;

    set("TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_IGNORE", "test");
    let mut exists_value = exists("TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_IGNORE");
    assert!(exists_value);

    set_list_with_options(
        "TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_IGNORE",
        &vec![],
        &options,
    );

    exists_value = exists("TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_IGNORE");
    assert!(!exists_value);

    options.ignore_empty = false;
    set_list_with_options(
        "TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_IGNORE",
        &vec![],
        &options,
    );

    options.ignore_empty = true;
    let output =
        get_list_with_options("TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_IGNORE", &options).unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output, vec!["".to_string()]);
}

#[test]
fn get_list_with_options_single_empty_no_ignore() {
    let mut options = ListOptions::new();
    options.ignore_empty = false;

    set_list_with_options(
        "TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_NO_IGNORE",
        &vec![],
        &options,
    );

    let output = get_list_with_options(
        "TEST_GET_LIST_WITH_OPTIONS_SINGLE_EMPTY_NO_IGNORE",
        &options,
    )
    .unwrap();
    assert_eq!(output.len(), 0);
}

#[test]
fn get_list_with_options_single() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options(
        "TEST_GET_LIST_WITH_OPTIONS_SINGLE",
        &vec!["1".to_string()],
        &options,
    );

    let output = get_list_with_options("TEST_GET_LIST_WITH_OPTIONS_SINGLE", &options).unwrap();
    assert_eq!(output.len(), 1);
    assert_eq!(output, vec!["1".to_string()]);
}

#[test]
fn get_list_with_options_multiple() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options(
        "TEST_GET_LIST_WITH_OPTIONS_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        &options,
    );

    let output = get_list_with_options("TEST_GET_LIST_WITH_OPTIONS_MULTIPLE", &options).unwrap();
    assert_eq!(output.len(), 3);
    assert_eq!(
        output,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
}

#[test]
fn expand_unix_prefix_none() {
    let output = expand("some text here", ExpansionType::UnixPrefix);

    assert_eq!("some text here", output);
}

#[test]
fn expand_unix_prefix_with_values() {
    set("TEST_EXPAND_UNIX_PREFIX_WITH_VALUES1", "test1");
    set("TEST_EXPAND_UNIX_PREFIX_WITH_VALUES2", "test2");
    set("TEST_EXPAND_UNIX_PREFIX_WITH_VALUES3", "test3");
    set("TEST_EXPAND_UNIX_PREFIX_WITH_VALUES4", "test4");
    set("TEST_EXPAND_UNIX_PREFIX_WITH_VALUES5", "test5");

    let output = expand(
        r#"
value1: $TEST_EXPAND_UNIX_PREFIX_WITH_VALUES1
value2: $TEST_EXPAND_UNIX_PREFIX_WITH_VALUES2
value3: ${TEST_EXPAND_UNIX_PREFIX_WITH_VALUES3}
value4: %TEST_EXPAND_UNIX_PREFIX_WITH_VALUES4%
value5: $TEST_EXPAND_UNIX_PREFIX_WITH_VALUES5"#,
        ExpansionType::UnixPrefix,
    );

    assert_eq!(
        r#"
value1: test1
value2: test2
value3: ${TEST_EXPAND_UNIX_PREFIX_WITH_VALUES3}
value4: %TEST_EXPAND_UNIX_PREFIX_WITH_VALUES4%
value5: test5"#,
        output
    );
}

#[test]
fn expand_unix_brackets_none() {
    let output = expand("some text here", ExpansionType::UnixBrackets);

    assert_eq!("some text here", output);
}

#[test]
fn expand_unix_brackets_with_values() {
    set("TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES1", "test1");
    set("TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES2", "test2");
    set("TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES3", "test3");
    set("TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES4", "test4");
    set("TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES5", "test5");

    let output = expand(
        r#"
value1: ${TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES1}
value2: ${TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES2}
value3: $TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES3
value4: %TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES4%
value5: ${TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES5
    "#,
        ExpansionType::UnixBrackets,
    );

    assert_eq!(
        r#"
value1: test1
value2: test2
value3: $TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES3
value4: %TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES4%
value5: ${TEST_EXPAND_UNIX_BRACKETS_WITH_VALUES5
    "#,
        output
    );
}

#[test]
fn expand_unix_with_values() {
    set("TEST_EXPAND_UNIX_WITH_VALUES1", "test1");
    set("TEST_EXPAND_UNIX_WITH_VALUES2", "test2");
    set("TEST_EXPAND_UNIX_WITH_VALUES3", "test3");
    set("TEST_EXPAND_UNIX_WITH_VALUES4", "test4");
    set("TEST_EXPAND_UNIX_WITH_VALUES5", "test5");

    let output = expand(
        r#"
value1: $TEST_EXPAND_UNIX_WITH_VALUES1
value2: ${TEST_EXPAND_UNIX_WITH_VALUES2}
value3: ${TEST_EXPAND_UNIX_WITH_VALUES3}
value4: %TEST_EXPAND_UNIX_WITH_VALUES4%
value5: $TEST_EXPAND_UNIX_WITH_VALUES5"#,
        ExpansionType::Unix,
    );

    assert_eq!(
        r#"
value1: test1
value2: test2
value3: test3
value4: %TEST_EXPAND_UNIX_WITH_VALUES4%
value5: test5"#,
        output
    );
}

#[test]
fn expand_windows_none() {
    let output = expand("some text here", ExpansionType::Windows);

    assert_eq!("some text here", output);
}

#[test]
fn expand_windows_with_values() {
    set("TEST_EXPAND_WINDOWS_WITH_VALUES1", "test1");
    set("TEST_EXPAND_WINDOWS_WITH_VALUES2", "test2");
    set("TEST_EXPAND_WINDOWS_WITH_VALUES3", "test3");
    set("TEST_EXPAND_WINDOWS_WITH_VALUES4", "test4");
    set("TEST_EXPAND_WINDOWS_WITH_VALUES5", "test5");

    let output = expand(
        r#"
value1: %TEST_EXPAND_WINDOWS_WITH_VALUES1%
value2: %TEST_EXPAND_WINDOWS_WITH_VALUES2%
value3: $TEST_EXPAND_WINDOWS_WITH_VALUES3
value4: ${TEST_EXPAND_WINDOWS_WITH_VALUES4}
value5: %TEST_EXPAND_WINDOWS_WITH_VALUES5
    "#,
        ExpansionType::Windows,
    );

    assert_eq!(
        r#"
value1: test1
value2: test2
value3: $TEST_EXPAND_WINDOWS_WITH_VALUES3
value4: ${TEST_EXPAND_WINDOWS_WITH_VALUES4}
value5: %TEST_EXPAND_WINDOWS_WITH_VALUES5
    "#,
        output
    );
}

#[test]
#[cfg(not(target_os = "windows"))]
fn expand_os_with_values() {
    set("TEST_EXPAND_OS_WITH_VALUES1", "test1");
    set("TEST_EXPAND_OS_WITH_VALUES2", "test2");
    set("TEST_EXPAND_OS_WITH_VALUES3", "test3");
    set("TEST_EXPAND_OS_WITH_VALUES4", "test4");
    set("TEST_EXPAND_OS_WITH_VALUES5", "test5");

    let output = expand(
        r#"
value1: $TEST_EXPAND_OS_WITH_VALUES1
value2: ${TEST_EXPAND_OS_WITH_VALUES2}
value3: ${TEST_EXPAND_OS_WITH_VALUES3}
value4: %TEST_EXPAND_OS_WITH_VALUES4%
value5: $TEST_EXPAND_OS_WITH_VALUES5"#,
        ExpansionType::OS,
    );

    assert_eq!(
        r#"
value1: test1
value2: test2
value3: test3
value4: %TEST_EXPAND_OS_WITH_VALUES4%
value5: test5"#,
        output
    );
}

#[test]
fn expand_all_with_values() {
    set("TEST_EXPAND_ALL_WITH_VALUES1", "test1");
    set("TEST_EXPAND_ALL_WITH_VALUES2", "test2");
    set("TEST_EXPAND_ALL_WITH_VALUES3", "test3");
    set("TEST_EXPAND_ALL_WITH_VALUES4", "test4");
    set("TEST_EXPAND_ALL_WITH_VALUES5", "test5");

    let output = expand(
        r#"
value1: $TEST_EXPAND_ALL_WITH_VALUES1
value2: ${TEST_EXPAND_ALL_WITH_VALUES2}
value3: ${TEST_EXPAND_ALL_WITH_VALUES3}
value4: %TEST_EXPAND_ALL_WITH_VALUES4%
value5: $TEST_EXPAND_ALL_WITH_VALUES5"#,
        ExpansionType::All,
    );

    assert_eq!(
        r#"
value1: test1
value2: test2
value3: test3
value4: test4
value5: test5"#,
        output
    );
}

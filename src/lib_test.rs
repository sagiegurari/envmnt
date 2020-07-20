use super::*;
use doc_comment as _;
use rusty_hook as _;
use std::env;

#[test]
fn exists_false() {
    let output = exists("TEST_LIB_EXISTS_FALSE");
    assert!(!output);
}

#[test]
fn exists_true() {
    env::set_var("TEST_LIB_EXISTS_TRUE", "EXISTS");
    let output = exists("TEST_LIB_EXISTS_TRUE");
    assert!(output);
}

#[test]
fn remove_exists() {
    env::set_var("TEST_LIB_REMOVE_EXISTS", "EXISTS");
    remove("TEST_LIB_REMOVE_EXISTS");
    let output = exists("TEST_LIB_REMOVE_EXISTS");
    assert!(!output);
}

#[test]
fn remove_not_exists() {
    remove("TEST_LIB_REMOVE_NOT_EXISTS");
}

#[test]
fn get_remove_not_exists() {
    let output = get_remove("TEST_LIB_GET_REMOVE_NOT_EXISTS");
    assert!(output.is_none());
}

#[test]
fn get_remove_exists() {
    env::set_var("TEST_LIB_GET_REMOVE_EXISTS", "OLD");
    let output = get_remove("TEST_LIB_GET_REMOVE_EXISTS");
    assert_eq!(output.unwrap(), "OLD".to_string());
}

#[test]
fn remove_all_exists() {
    env::set_var("TEST_LIB_REMOVE_ALL_EXISTS1", "EXISTS1");
    env::set_var("TEST_LIB_REMOVE_ALL_EXISTS2", "EXISTS2");
    remove_all(&vec!["TEST_LIB_REMOVE_EXISTS1", "TEST_LIB_REMOVE_EXISTS2"]);
    let mut output = exists("TEST_LIB_REMOVE_EXISTS1");
    assert!(!output);
    output = exists("TEST_LIB_REMOVE_EXISTS2");
    assert!(!output);
}

#[test]
fn get_or_exists() {
    env::set_var("TEST_LIB_GET_OR_EXISTS", "EXISTS");
    let output = get_or("TEST_LIB_GET_OR_EXISTS", "bad");
    assert_eq!(output, "EXISTS".to_string());
}

#[test]
fn get_or_not_exists() {
    let output = get_or("TEST_LIB_GET_OR_NOT_EXISTS", "good");
    assert_eq!(output, "good".to_string());
}

#[test]
fn get_or_empty() {
    env::set_var("TEST_LIB_GET_OR_EMPTY", "");
    let output = get_or("TEST_LIB_GET_OR_EMPTY", "bad");
    assert_eq!(output, "".to_string());
}

#[test]
fn get_or_panic_exists() {
    env::set_var("TEST_LIB_GET_OR_PANIC_EXISTS", "EXISTS");
    let output = get_or_panic("TEST_LIB_GET_OR_PANIC_EXISTS");
    assert_eq!(output, "EXISTS".to_string());
}

#[test]
#[should_panic]
fn get_or_panic_not_exists() {
    get_or_panic("TEST_LIB_GET_OR_PANIC_NOT_EXISTS");
}

#[test]
fn get_any_exists() {
    env::set_var("TEST_LIB_GET_ANY_EXISTS2", "EXISTS2");
    let output = get_any(
        &vec!["TEST_LIB_GET_ANY_EXISTS1", "TEST_LIB_GET_ANY_EXISTS2"],
        "not found",
    );
    assert_eq!(output, "EXISTS2".to_string());
}

#[test]
fn get_any_not_exists() {
    let output = get_any(
        &vec![
            "TEST_LIB_GET_ANY_NOT_EXISTS1",
            "TEST_LIB_GET_ANY_NOT_EXISTS2",
        ],
        "not found",
    );
    assert_eq!(output, "not found".to_string());
}

#[test]
fn is_or_false() {
    env::set_var("TEST_LIB_IS_OR_BOOL_FALSE", "false");
    let output = is_or("TEST_LIB_IS_OR_BOOL_FALSE", true);
    assert!(!output);
}

#[test]
fn is_or_false_uppercase() {
    env::set_var("TEST_LIB_IS_OR_BOOL_FALSE_UPPER", "FALSE");
    let output = is_or("TEST_LIB_IS_OR_BOOL_FALSE_UPPER", true);
    assert!(!output);
}

#[test]
fn is_or_no() {
    env::set_var("TEST_LIB_IS_OR_BOOL_NO", "no");
    let output = is_or("TEST_LIB_IS_OR_BOOL_NO", true);
    assert!(!output);
}

#[test]
fn is_or_no_uppercase() {
    env::set_var("TEST_LIB_IS_OR_BOOL_NO_UPPER", "NO");
    let output = is_or("TEST_LIB_IS_OR_BOOL_NO_UPPER", true);
    assert!(!output);
}

#[test]
fn is_or_zero() {
    env::set_var("TEST_LIB_IS_OR_BOOL_ZERO", "0");
    let output = is_or("TEST_LIB_IS_OR_BOOL_ZERO", true);
    assert!(!output);
}

#[test]
fn is_or_empty() {
    env::set_var("TEST_LIB_IS_OR_BOOL_EMPTY", "");
    let output = is_or("TEST_LIB_IS_OR_BOOL_EMPTY", false);
    assert!(!output);
}

#[test]
fn is_or_else() {
    env::set_var("TEST_LIB_IS_OR_BOOL_ELSE", "true");
    let output = is_or("TEST_LIB_IS_OR_BOOL_ELSE", false);
    assert!(output);
}

#[test]
fn is_or_default_true() {
    let output = is_or("TEST_LIB_IS_OR_BOOL_NO_EXISTS_TRUE", true);
    assert!(output);
}

#[test]
fn is_or_default_false() {
    let output = is_or("TEST_LIB_IS_OR_BOOL_NO_EXISTS_FALSE", false);
    assert!(!output);
}

#[test]
fn is_false() {
    env::set_var("TEST_LIB_IS_BOOL_FALSE", "false");
    let output = is("TEST_LIB_IS_BOOL_FALSE");
    assert!(!output);
}

#[test]
fn is_empty() {
    env::set_var("TEST_LIB_IS_BOOL_EMPTY", "");
    let output = is("TEST_LIB_IS_BOOL_EMPTY");
    assert!(!output);
}

#[test]
fn is_true() {
    env::set_var("TEST_LIB_IS_BOOL_TRUE", "true");
    let output = is("TEST_LIB_IS_BOOL_TRUE");
    assert!(output);
}

#[test]
fn is_undefined() {
    let output = is("TEST_LIB_IS_BOOL_UNDEFINED");
    assert!(!output);
}

#[test]
fn set_value() {
    set("TEST_LIB_SET_VALUE", "SIMPLE");
    assert_eq!(
        env::var("TEST_LIB_SET_VALUE").unwrap(),
        "SIMPLE".to_string()
    );
}

#[test]
fn set_bool_false() {
    set_bool("TEST_LIB_SET_BOOL_FALSE", false);
    assert_eq!(
        env::var("TEST_LIB_SET_BOOL_FALSE").unwrap(),
        "false".to_string()
    );
}

#[test]
fn set_bool_true() {
    set_bool("TEST_LIB_SET_BOOL_TRUE", true);
    assert_eq!(
        env::var("TEST_LIB_SET_BOOL_TRUE").unwrap(),
        "true".to_string()
    );
}

#[test]
fn set_optional_none() {
    let output = set_optional::<&str, &str>("TEST_LIB_SET_OPTIONAL_NONE", &None);

    assert!(!output);
}

#[test]
fn set_optional_some() {
    let output = set_optional("TEST_LIB_SET_OPTIONAL_SOME", &Some("OPTIONAL VALUE"));

    assert!(output);
    assert_eq!(
        env::var("TEST_LIB_SET_OPTIONAL_SOME").unwrap(),
        "OPTIONAL VALUE".to_string()
    );
}

#[test]
fn set_or_remove_none() {
    let output = set_or_remove::<&str, &str>("TEST_LIB_SET_OR_REMOVE_NONE", &None);

    assert!(!output);
}

#[test]
fn set_or_remove_some() {
    let mut output = set_or_remove("TEST_LIB_SET_OR_REMOVE_SOME", &Some("OPTIONAL VALUE"));

    assert!(output);
    assert_eq!(
        env::var("TEST_LIB_SET_OR_REMOVE_SOME").unwrap(),
        "OPTIONAL VALUE".to_string()
    );

    output = set_or_remove::<&str, &str>("TEST_LIB_SET_OR_REMOVE_SOME", &None);

    assert!(!output);
    assert!(!exists("TEST_LIB_SET_OR_REMOVE_SOME"));
}

#[test]
fn get_set_exists() {
    env::set_var("TEST_LIB_GET_SET_EXISTS", "OLD");
    let output = get_set("TEST_LIB_GET_SET_EXISTS", "NEW");
    assert_eq!(output.unwrap(), "OLD".to_string());
    assert_eq!(
        env::var("TEST_LIB_GET_SET_EXISTS").unwrap(),
        "NEW".to_string()
    );
}

#[test]
fn get_set_not_exists() {
    let output = get_set("TEST_LIB_GET_SET_NOT_EXISTS", "NEW");
    assert!(output.is_none());
    assert_eq!(
        env::var("TEST_LIB_GET_SET_NOT_EXISTS").unwrap(),
        "NEW".to_string()
    );
}

#[test]
fn vars_valid() {
    let output = vars();
    assert!(output.len() > 0);
}

#[test]
fn is_equal_not_exists() {
    let output = is_equal("TEST_LIB_IS_EQUAL_NOT_EXISTS", "VALUE");
    assert!(!output);
}

#[test]
fn is_equal_same() {
    env::set_var("TEST_LIB_IS_EQUAL_SAME", "VALUE");
    let output = is_equal("TEST_LIB_IS_EQUAL_SAME", "VALUE");
    assert!(output);
}

#[test]
fn is_equal_not_same() {
    env::set_var("TEST_LIB_IS_EQUAL_NOT_SAME", "1");
    let output = is_equal("TEST_LIB_IS_EQUAL_NOT_SAME", "2");
    assert!(!output);
}

#[test]
fn contains_not_exists() {
    let output = contains("TEST_LIB_CONTAINS_NOT_EXISTS", "VALUE");
    assert!(!output);
}

#[test]
fn contains_same() {
    env::set_var("TEST_LIB_CONTAINS_SAME", "VALUE");
    let output = contains("TEST_LIB_CONTAINS_SAME", "VAL");
    assert!(output);
}

#[test]
fn contains_not_same() {
    env::set_var("TEST_LIB_CONTAINS_NOT_SAME", "VALUE");
    let output = contains("TEST_LIB_CONTAINS_NOT_SAME", "val");
    assert!(!output);
}

#[test]
fn contains_ignore_case_not_exists() {
    let output = contains_ignore_case("TEST_LIB_CONTAINS_IGNORE_CASE_NOT_EXISTS", "VALUE");
    assert!(!output);
}

#[test]
fn contains_ignore_case_same() {
    env::set_var("TEST_LIB_CONTAINS_IGNORE_CASE_SAME", "VALUE");
    let output = contains_ignore_case("TEST_LIB_CONTAINS_IGNORE_CASE_SAME", "VAL");
    assert!(output);
}

#[test]
fn contains_ignore_case_same_value_different_case() {
    env::set_var("TEST_LIB_CONTAINS_IGNORE_CASE_SAME_DIFFERENT_CASE", "VALUE");
    let output = contains_ignore_case("TEST_LIB_CONTAINS_IGNORE_CASE_SAME_DIFFERENT_CASE", "val");
    assert!(output);
}

#[test]
fn contains_ignore_case_not_same() {
    env::set_var("TEST_LIB_CONTAINS_IGNORE_CASE_NOT_SAME", "1");
    let output = contains_ignore_case("TEST_LIB_CONTAINS_IGNORE_CASE_NOT_SAME", "2");
    assert!(!output);
}

#[test]
fn set_list_multiple() {
    set_list(
        "TEST_LIB_SET_LIST_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let output = is_equal("TEST_LIB_SET_LIST_MULTIPLE", "1;2;3");
    assert!(output);
}

#[test]
fn get_list_none() {
    let output = get_list("TEST_LIB_GET_LIST_NONE").is_none();
    assert!(output);
}

#[test]
fn get_list_multiple() {
    set_list(
        "TEST_LIB_GET_LIST_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let output = get_list("TEST_LIB_GET_LIST_MULTIPLE").unwrap();
    assert_eq!(output.len(), 3);
    assert_eq!(
        output,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
}

#[test]
fn set_list_with_options_multiple() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options(
        "TEST_LIB_SET_LIST_WITH_OPTIONS_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        &options,
    );

    let output = is_equal("TEST_LIB_SET_LIST_WITH_OPTIONS_MULTIPLE", "1,2,3");
    assert!(output);
}

#[test]
fn get_list_with_options_none() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    let output = get_list_with_options("TEST_LIB_GET_LIST_WITH_OPTIONS_NONE", &options).is_none();
    assert!(output);
}

#[test]
fn get_list_with_options_multiple() {
    let mut options = ListOptions::new();
    options.separator = Some(",".to_string());

    set_list_with_options(
        "TEST_LIB_GET_LIST_WITH_OPTIONS_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        &options,
    );

    let output =
        get_list_with_options("TEST_LIB_GET_LIST_WITH_OPTIONS_MULTIPLE", &options).unwrap();
    assert_eq!(output.len(), 3);
    assert_eq!(
        output,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );
}

#[test]
fn set_all_valid() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert("TEST_LIB_SET_ALL_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("TEST_LIB_SET_ALL_VAR2".to_string(), "MY VALUE2".to_string());

    set_all(&env);

    let mut output = environment::is_equal("TEST_LIB_SET_ALL_VAR1", "MY VALUE");
    assert!(output);
    output = environment::is_equal("TEST_LIB_SET_ALL_VAR2", "MY VALUE2");
    assert!(output);
}

#[test]
fn evaluate_and_set_all_valid() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert(
        "TEST_LIB_EVAL_AND_SET_ALL_VAR1".to_string(),
        "MY VALUE".to_string(),
    );
    env.insert(
        "TEST_LIB_EVAL_AND_SET_ALL_VAR2".to_string(),
        "MY VALUE2".to_string(),
    );

    let eval_env = |value: String| {
        let mut buffer = String::from("VALUE-");
        buffer.push_str(&value);
        buffer
    };

    evaluate_and_set_all(&env, eval_env);

    let mut output = environment::is_equal("TEST_LIB_EVAL_AND_SET_ALL_VAR1", "VALUE-MY VALUE");
    assert!(output);
    output = environment::is_equal("TEST_LIB_EVAL_AND_SET_ALL_VAR2", "VALUE-MY VALUE2");
    assert!(output);
}

#[test]
fn is_any_exists_found() {
    env::set_var("TEST_LIB_ANY_EXISTS_FOUND1", "EMPTY");

    let found = is_any_exists(&vec![
        "TEST_LIB_ANY_EXISTS_FOUND1",
        "TEST_LIB_ANY_EXISTS_FOUND2",
    ]);

    assert!(found);
}

#[test]
fn is_all_exists_found() {
    env::set_var("TEST_LIB_ALL_EXISTS_FOUND1", "EMPTY");
    env::set_var("TEST_LIB_ALL_EXISTS_FOUND2", "EMPTY");

    let found = is_all_exists(&vec![
        "TEST_LIB_ALL_EXISTS_FOUND1",
        "TEST_LIB_ALL_EXISTS_FOUND2",
    ]);

    assert!(found);
}

#[test]
fn load_file_valid() {
    let output = load_file("./src/test/var.env");

    assert!(output.is_ok());

    assert_eq!(get_or_panic("var1"), "value1");
    assert_eq!(get_or_panic("var2"), "value2");
    assert_eq!(get_or_panic("var3"), "==value3==");
}

#[test]
fn evaluate_and_load_file_no_evaluation() {
    let output = evaluate_and_load_file("./src/test/var.env", file::empty_evaluate_fn);

    assert!(output.is_ok());

    assert_eq!(get_or_panic("var1"), "value1");
    assert_eq!(get_or_panic("var2"), "value2");
    assert_eq!(get_or_panic("var3"), "==value3==");
}

#[test]
fn parse_file_valid() {
    let output = parse_file("./src/test/parse.env").unwrap();

    assert!(output.len() == 3);
    assert_eq!(output.get("key1").unwrap(), "value1");
    assert_eq!(output.get("key2").unwrap(), "value2");
    assert_eq!(output.get("key3").unwrap(), "==value3==");
}

#[test]
fn expand_all_with_values() {
    set("TEST_LIB_EXPAND_ALL_WITH_VALUES1", "test1");
    set("TEST_LIB_EXPAND_ALL_WITH_VALUES2", "test2");
    set("TEST_LIB_EXPAND_ALL_WITH_VALUES3", "test3");
    set("TEST_LIB_EXPAND_ALL_WITH_VALUES4", "test4");
    set("TEST_LIB_EXPAND_ALL_WITH_VALUES5", "test5");

    let mut options = ExpandOptions::new();
    options.expansion_type = Some(ExpansionType::All);
    options.default_to_empty = false;

    let output = expand(
        r#"
value1: $TEST_LIB_EXPAND_ALL_WITH_VALUES1
value2: ${TEST_LIB_EXPAND_ALL_WITH_VALUES2}
value3: ${TEST_LIB_EXPAND_ALL_WITH_VALUES3}
value4: %TEST_LIB_EXPAND_ALL_WITH_VALUES4%
value5: $TEST_LIB_EXPAND_ALL_WITH_VALUES5"#,
        Some(options),
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

#[test]
#[cfg(not(target_os = "windows"))]
fn expand_none_options() {
    set("TEST_LIB_EXPAND_NONE_OPTIONS1", "test1");
    set("TEST_LIB_EXPAND_NONE_OPTIONS2", "test2");
    set("TEST_LIB_EXPAND_NONE_OPTIONS3", "test3");
    set("TEST_LIB_EXPAND_NONE_OPTIONS4", "test4");
    set("TEST_LIB_EXPAND_NONE_OPTIONS5", "test5");

    let output = expand(
        r#"
value1:$TEST_LIB_EXPAND_NONE_OPTIONS1
value2:${TEST_LIB_EXPAND_NONE_OPTIONS2}
value3:${TEST_LIB_EXPAND_NONE_OPTIONS3}
value4:%TEST_LIB_EXPAND_NONE_OPTIONS4%
value5:$TEST_LIB_EXPAND_NONE_OPTIONS5
value6:${TEST_LIB_EXPAND_NONE_OPTIONS6}"#,
        None,
    );

    assert_eq!(
        r#"
value1:test1
value2:test2
value3:test3
value4:%TEST_LIB_EXPAND_NONE_OPTIONS4%
value5:test5
value6:"#,
        output
    );
}

#[test]
fn get_number_undefined() {
    let output_u8 = get_u8("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u8, 5);
    let output_i8 = get_i8("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i8, 5);
    let output_u16 = get_u16("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u16, 5);
    let output_i16 = get_i16("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i16, 5);
    let output_u32 = get_u32("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u32, 5);
    let output_i32 = get_i32("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i32, 5);
    let output_u64 = get_u64("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u64, 5);
    let output_i64 = get_i64("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i64, 5);
    let output_u128 = get_u128("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_u128, 5);
    let output_i128 = get_i128("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_i128, 5);
    let output_f32 = get_f32("TEST_LIB_GET_NUMBER_UNDEFINED", 5.0);
    assert_eq!(output_f32, 5.0);
    let output_f64 = get_f64("TEST_LIB_GET_NUMBER_UNDEFINED", 5.0);
    assert_eq!(output_f64, 5.0);
    let output_usize = get_usize("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_usize, 5);
    let output_isize = get_isize("TEST_LIB_GET_NUMBER_UNDEFINED", 5);
    assert_eq!(output_isize, 5);
}

#[test]
fn get_number_invalid() {
    env::set_var("TEST_LIB_GET_NUMBER_INVALID", "abc");
    let output_u8 = get_u8("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u8, 5);
    let output_i8 = get_i8("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i8, 5);
    let output_u16 = get_u16("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u16, 5);
    let output_i16 = get_i16("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i16, 5);
    let output_u32 = get_u32("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u32, 5);
    let output_i32 = get_i32("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i32, 5);
    let output_u64 = get_u64("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u64, 5);
    let output_i64 = get_i64("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i64, 5);
    let output_u128 = get_u128("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_u128, 5);
    let output_i128 = get_i128("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_i128, 5);
    let output_f32 = get_f32("TEST_LIB_GET_NUMBER_INVALID", 5.0);
    assert_eq!(output_f32, 5.0);
    let output_f64 = get_f64("TEST_LIB_GET_NUMBER_INVALID", 5.0);
    assert_eq!(output_f64, 5.0);
    let output_usize = get_usize("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_usize, 5);
    let output_isize = get_isize("TEST_LIB_GET_NUMBER_INVALID", 5);
    assert_eq!(output_isize, 5);
}

#[test]
fn get_number_valid() {
    env::set_var("TEST_LIB_GET_NUMBER_VALID", 15.to_string());
    let output_u8 = get_u8("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_u8, 15);
    let output_i8 = get_i8("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_i8, 15);
    let output_u16 = get_u16("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_u16, 15);
    let output_i16 = get_i16("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_i16, 15);
    let output_u32 = get_u32("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_u32, 15);
    let output_i32 = get_i32("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_i32, 15);
    let output_u64 = get_u64("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_u64, 15);
    let output_i64 = get_i64("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_i64, 15);
    let output_u128 = get_u128("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_u128, 15);
    let output_i128 = get_i128("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_i128, 15);
    let output_f32 = get_f32("TEST_LIB_GET_NUMBER_VALID", 5.0);
    assert_eq!(output_f32, 15.0);
    let output_f64 = get_f64("TEST_LIB_GET_NUMBER_VALID", 5.0);
    assert_eq!(output_f64, 15.0);
    let output_usize = get_usize("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_usize, 15);
    let output_isize = get_isize("TEST_LIB_GET_NUMBER_VALID", 5);
    assert_eq!(output_isize, 15);
}

#[test]
fn set_number_valid() {
    set_u8("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_u8 = get_u8("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_u8, 15);
    set_i8("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_i8 = get_i8("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_i8, 15);
    set_u16("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_u16 = get_u16("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_u16, 15);
    set_i16("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_i16 = get_i16("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_i16, 15);
    set_u32("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_u32 = get_u32("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_u32, 15);
    set_i32("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_i32 = get_i32("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_i32, 15);
    set_u64("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_u64 = get_u64("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_u64, 15);
    set_i64("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_i64 = get_i64("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_i64, 15);
    set_u128("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_u128 = get_u128("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_u128, 15);
    set_i128("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_i128 = get_i128("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_i128, 15);
    set_f32("TEST_LIB_SET_NUMBER_VALID", 15.0);
    let output_f32 = get_f32("TEST_LIB_SET_NUMBER_VALID", 5.0);
    assert_eq!(output_f32, 15.0);
    set_f64("TEST_LIB_SET_NUMBER_VALID", 15.0);
    let output_f64 = get_f64("TEST_LIB_SET_NUMBER_VALID", 5.0);
    assert_eq!(output_f64, 15.0);
    set_usize("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_usize = get_usize("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_usize, 15);
    set_isize("TEST_LIB_SET_NUMBER_VALID", 15);
    let output_isize = get_isize("TEST_LIB_SET_NUMBER_VALID", 5);
    assert_eq!(output_isize, 15);
}

#[test]
fn increment_valid() {
    set_u8("TEST_LIB_INCREMENT_VALID", 50);
    let value = increment("TEST_LIB_INCREMENT_VALID");
    assert_eq!(value, 51);
}

#[test]
fn decrement_valid() {
    set_u8("TEST_LIB_DECREMENT_VALID", 50);
    let value = decrement("TEST_LIB_DECREMENT_VALID");
    assert_eq!(value, 49);
}

use super::*;

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
fn set_list_with_separator_multiple() {
    set_list_with_separator(
        "TEST_LIB_SET_LIST_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        ",",
    );

    let output = is_equal("TEST_LIB_SET_LIST_MULTIPLE", "1,2,3");
    assert!(output);
}

#[test]
fn get_list_with_separator_none() {
    let output = get_list_with_separator("TEST_LIB_GET_LIST_NONE", ",").is_none();
    assert!(output);
}

#[test]
fn get_list_with_separator_multiple() {
    set_list_with_separator(
        "TEST_LIB_GET_LIST_MULTIPLE",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        ",",
    );

    let output = get_list_with_separator("TEST_LIB_GET_LIST_MULTIPLE", ",").unwrap();
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

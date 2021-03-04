use super::*;

use crate::environment;
use fsio::error::ErrorInfo;

#[test]
fn load_file_valid() {
    let output = load_file("./src/test/load.env");

    assert!(output.is_ok());

    assert_eq!(environment::get_or_panic("load1"), "value1");
    assert_eq!(environment::get_or_panic("load2"), "value2");
    assert_eq!(environment::get_or_panic("load3"), "==value3==");
    assert_eq!(environment::get_or_panic("load4"), "value4");
    assert_eq!(environment::get_or_panic("load5"), "value5");
    assert_eq!(environment::get_or_panic("load6"), "==value\"6\"==");
    assert_eq!(environment::get_or_panic("load7"), "value\"7\n");
    assert_eq!(environment::get_or_panic("load8"), "value\r\"8");
    assert_eq!(environment::get_or_panic("load9"), "\"value9");
    assert_eq!(environment::get_or_panic("load10"), "value10\"");
    assert_eq!(environment::get_or_panic("load11"), "\"value11\"");
    assert_eq!(environment::get_or_panic("load12"), "\"");
    assert_eq!(environment::get_or_panic("load13"), "\"\"");
    assert_eq!(environment::get_or_panic("load14"), "\"value14");
    assert_eq!(environment::get_or_panic("load15"), "value15\"");
    assert_eq!(environment::get_or_panic("load16"), "");
    assert_eq!(environment::get_or_panic("load17"), "");
}

#[test]
fn load_file_not_found() {
    let output = load_file("./src/test/bad.env");

    assert!(output.is_err());
}

#[test]
fn evaluate_and_load_file_no_evaluation() {
    let output = evaluate_and_load_file("./src/test/eval.env", empty_evaluate_fn);

    assert!(output.is_ok());

    assert_eq!(environment::get_or_panic("eval1"), "value1");
    assert_eq!(environment::get_or_panic("eval2"), "value2");
    assert_eq!(environment::get_or_panic("eval3"), "==value3==");
}

#[test]
fn evaluate_and_load_file_with_evaluation() {
    let test_evaluate_fn = |value: String| {
        let mut buffer = String::from("EVAL-");
        buffer.push_str(&value);
        buffer
    };

    let output = evaluate_and_load_file("./src/test/eval2.env", test_evaluate_fn);

    assert!(output.is_ok());

    assert_eq!(environment::get_or_panic("eval_test_1"), "EVAL-value1");
    assert_eq!(environment::get_or_panic("eval_test_2"), "EVAL-value2");
    assert_eq!(environment::get_or_panic("eval_test_3"), "EVAL-==value3==");
}

#[test]
fn evaluate_and_load_file_not_found() {
    let output = evaluate_and_load_file("./src/test/bad.env", empty_evaluate_fn);

    assert!(output.is_err());
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
fn parse_file_not_found() {
    let output = parse_file("./src/test/bad.env");

    assert!(output.is_err());
}

#[test]
fn create_read_file_error_valid() {
    let error = create_read_file_error(FsIOError {
        info: ErrorInfo::PathAlreadyExists("test".to_string()),
    });

    let error_string = error.to_string();
    assert!(error_string.contains("Unable to read file."));
    assert!(error_string.contains("test"));
}

#[test]
fn parse_env_file_content_empty() {
    let output = parse_env_file_content("");

    assert!(output.len() == 0);
}

#[test]
fn parse_env_file_content_comment_strings() {
    let content = r#"
        # test comment line

        # another comment line
    "#;
    let output = parse_env_file_content(&content);

    assert!(output.len() == 0);
}

#[test]
fn parse_env_file_content_valid() {
    let content = r#"
        # test comment line
        key1=value1
        key2 = value2
        key3 = ==value3==

        # another comment line
    "#;
    let output = parse_env_file_content(&content);

    assert!(output.len() == 3);
    assert_eq!(output.get("key1").unwrap(), "value1");
    assert_eq!(output.get("key2").unwrap(), "value2");
    assert_eq!(output.get("key3").unwrap(), "==value3==");
}

use super::*;

use crate::environment;
use std::io::ErrorKind;

#[test]
fn load_file_valid() {
    let output = load_file("./src/test/load.env");

    assert!(output.is_ok());

    assert_eq!(environment::get_or_panic("load1"), "value1");
    assert_eq!(environment::get_or_panic("load2"), "value2");
    assert_eq!(environment::get_or_panic("load3"), "==value3==");
}

#[test]
fn load_file_not_found() {
    let output = load_file("./src/test/bad.env");

    assert!(output.is_err());

    match output {
        Err(error) => assert!(error.to_string().contains("./src/test/bad.env")),
        _ => panic!("Invalid Case"),
    };
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

    match output {
        Err(error) => assert!(error.to_string().contains("./src/test/bad.env")),
        _ => panic!("Invalid Case"),
    };
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

    match output {
        Err(error) => assert!(error.to_string().contains("./src/test/bad.env")),
        _ => panic!("Invalid Case"),
    };
}

#[test]
fn create_file_open_error_valid() {
    let io_error = Error::new(ErrorKind::Other, "test");
    let output = create_file_open_error("./src/test/bad.env", io_error);

    assert!(output.to_string().contains("./src/test/bad.env"));
}

#[test]
fn parse_env_content_empty() {
    let output = parse_env_content("");

    assert!(output.len() == 0);
}

#[test]
fn parse_env_content_comment_strings() {
    let content = r#"
        # test comment line

        # another comment line
    "#;
    let output = parse_env_content(&content);

    assert!(output.len() == 0);
}

#[test]
fn parse_env_content_valid() {
    let content = r#"
        # test comment line
        key1=value1
        key2 = value2
        key3 = ==value3==

        # another comment line
    "#;
    let output = parse_env_content(&content);

    assert!(output.len() == 3);
    assert_eq!(output.get("key1").unwrap(), "value1");
    assert_eq!(output.get("key2").unwrap(), "value2");
    assert_eq!(output.get("key3").unwrap(), "==value3==");
}

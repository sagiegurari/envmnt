use super::*;

use std::env;

#[test]
fn remove_all_not_exists() {
    environment::remove("TEST_REMOVE_ALL_NOT_EXISTS1");
    environment::remove("TEST_REMOVE_ALL_NOT_EXISTS2");
    remove_all(&vec!["TEST_REMOVE_NOT_EXISTS1", "TEST_REMOVE_NOT_EXISTS2"]);
    let mut output = environment::exists("TEST_REMOVE_NOT_EXISTS1");
    assert!(!output);
    output = environment::exists("TEST_REMOVE_NOT_EXISTS2");
    assert!(!output);
}

#[test]
fn remove_all_exists() {
    env::set_var("TEST_REMOVE_ALL_EXISTS1", "EXISTS1");
    env::set_var("TEST_REMOVE_ALL_EXISTS2", "EXISTS2");
    remove_all(&vec!["TEST_REMOVE_EXISTS1", "TEST_REMOVE_EXISTS2"]);
    let mut output = environment::exists("TEST_REMOVE_EXISTS1");
    assert!(!output);
    output = environment::exists("TEST_REMOVE_EXISTS2");
    assert!(!output);
}

#[test]
fn remove_all_partial_exists() {
    env::set_var("TEST_REMOVE_ALL_PARTIAL_EXISTS1", "EXISTS1");
    environment::remove("TEST_REMOVE_ALL_PARTIAL_EXISTS2");
    remove_all(&vec![
        "TEST_REMOVE_PARTIAL_EXISTS1",
        "TEST_REMOVE_PARTIAL_EXISTS2",
    ]);
    let mut output = environment::exists("TEST_REMOVE_PARTIAL_EXISTS1");
    assert!(!output);
    output = environment::exists("TEST_REMOVE_PARTIAL_EXISTS2");
    assert!(!output);
}

#[test]
fn set_all_valid() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert("SET_ALL_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("SET_ALL_VAR2".to_string(), "MY VALUE2".to_string());

    set_all(&env);

    let mut output = environment::is_equal("SET_ALL_VAR1", "MY VALUE");
    assert!(output);
    output = environment::is_equal("SET_ALL_VAR2", "MY VALUE2");
    assert!(output);
}

#[test]
fn evaluate_and_set_all_valid() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert(
        "TEST_EVAL_AND_SET_ALL_VAR1".to_string(),
        "MY VALUE".to_string(),
    );
    env.insert(
        "TEST_EVAL_AND_SET_ALL_VAR2".to_string(),
        "MY VALUE2".to_string(),
    );

    let eval_env = |value: String| {
        let mut buffer = String::from("VALUE-");
        buffer.push_str(&value);
        buffer
    };

    evaluate_and_set_all(&env, eval_env);

    let mut output = environment::is_equal("TEST_EVAL_AND_SET_ALL_VAR1", "VALUE-MY VALUE");
    assert!(output);
    output = environment::is_equal("TEST_EVAL_AND_SET_ALL_VAR2", "VALUE-MY VALUE2");
    assert!(output);
}

#[test]
fn is_any_exists_empty() {
    let vars: Vec<String> = vec![];
    let found = is_any_exists(&vars);

    assert!(!found);
}

#[test]
fn is_any_exists_found() {
    env::set_var("TEST_ANY_EXISTS_FOUND1", "EMPTY");

    let found = is_any_exists(&vec!["TEST_ANY_EXISTS_FOUND1", "TEST_ANY_EXISTS_FOUND2"]);

    assert!(found);
}

#[test]
fn is_any_exists_not_found() {
    let found = is_any_exists(&vec!["TEST_ANY_EXISTS_NOT_FOUND"]);

    assert!(!found);
}

#[test]
fn is_all_exists_empty() {
    let vars: Vec<String> = vec![];
    let found = is_all_exists(&vars);

    assert!(!found);
}

#[test]
fn is_all_exists_found() {
    env::set_var("TEST_ALL_EXISTS_FOUND1", "EMPTY");
    env::set_var("TEST_ALL_EXISTS_FOUND2", "EMPTY");

    let found = is_all_exists(&vec!["TEST_ALL_EXISTS_FOUND1", "TEST_ALL_EXISTS_FOUND2"]);

    assert!(found);
}

#[test]
fn is_all_exists_partial() {
    env::set_var("TEST_ALL_EXISTS_PARTIAL1", "EMPTY");

    let found = is_all_exists(&vec![
        "TEST_ALL_EXISTS_PARTIAL1",
        "TEST_ALL_EXISTS_PARTIAL2",
    ]);

    assert!(!found);
}

#[test]
fn is_all_exists_not_found() {
    let found = is_all_exists(&vec![
        "TEST_ALL_EXISTS_NOT_FOUND1",
        "TEST_ALL_EXISTS_NOT_FOUND2",
    ]);

    assert!(!found);
}

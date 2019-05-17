use super::*;

use std::env;

#[test]
fn set_all_valid() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert("MY_ENV_VAR".to_string(), "MY VALUE".to_string());
    env.insert("MY_ENV_VAR2".to_string(), "MY VALUE2".to_string());

    set_all(&env);

    let mut output = environment::is_equal("MY_ENV_VAR", "MY VALUE");
    assert!(output);
    output = environment::is_equal("MY_ENV_VAR2", "MY VALUE2");
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

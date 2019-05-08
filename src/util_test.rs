use super::*;

#[test]
fn bool_to_string_true() {
    let output = bool_to_string(true);
    assert_eq!(output, "true".to_string());
}

#[test]
fn bool_to_string_false() {
    let output = bool_to_string(false);
    assert_eq!(output, "false".to_string());
}

#[test]
fn string_to_bool_empty() {
    let output = string_to_bool("");
    assert!(!output);
}

#[test]
fn string_to_bool_zero() {
    let output = string_to_bool("0");
    assert!(!output);
}

#[test]
fn string_to_bool_no_uppercase() {
    let output = string_to_bool("NO");
    assert!(!output);
}

#[test]
fn string_to_bool_no_lower_case() {
    let output = string_to_bool("no");
    assert!(!output);
}

#[test]
fn string_to_bool_false_uppercase() {
    let output = string_to_bool("FALSE");
    assert!(!output);
}

#[test]
fn string_to_bool_false_lower_case() {
    let output = string_to_bool("false");
    assert!(!output);
}

#[test]
fn string_to_bool_else() {
    let output = string_to_bool("any value");
    assert!(output);
}

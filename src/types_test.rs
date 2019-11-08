use super::*;

#[test]
fn list_options_new() {
    let options = ListOptions::new();

    assert!(options.separator.is_none());
    assert!(!options.ignore_empty);
}

#[test]
fn expand_options_new() {
    let options = ExpandOptions::new();

    assert!(options.expansion_type.is_none());
    assert!(options.default_to_empty);
}

#[test]
fn expand_options_clone_with_expansion_type() {
    let default_options = ExpandOptions::new();

    let options = default_options.clone_with_expansion_type(ExpansionType::UnixBrackets);

    assert!(options.expansion_type.is_some());
    assert_eq!(options.expansion_type.unwrap(), ExpansionType::UnixBrackets);
    assert!(options.default_to_empty);
}

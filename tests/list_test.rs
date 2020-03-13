use envmnt;

#[test]
fn get() {
    envmnt::set_list(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let mut values = envmnt::get_list("LIST_TEST_ENV").unwrap();
    assert_eq!(
        values,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );

    let mut same = envmnt::is_equal("LIST_TEST_ENV", "1;2;3");
    assert!(same);

    let mut options = envmnt::ListOptions::new();
    options.separator = Some(",".to_string());
    envmnt::set_list_with_options(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        &options,
    );

    values = envmnt::get_list_with_options("LIST_TEST_ENV", &options).unwrap();
    assert_eq!(
        values,
        vec!["1".to_string(), "2".to_string(), "3".to_string()]
    );

    same = envmnt::is_equal("LIST_TEST_ENV", "1,2,3");
    assert!(same);
}

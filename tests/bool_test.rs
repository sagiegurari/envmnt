use envmnt;

#[test]
fn get() {
    let mut exists = envmnt::exists("BOOL_TEST_ENV");
    assert!(!exists);

    envmnt::set_bool("BOOL_TEST_ENV", true);
    exists = envmnt::exists("BOOL_TEST_ENV");
    assert!(exists);

    let optional_value = envmnt::get_set("BOOL_TEST_ENV", "2");
    exists = envmnt::exists("BOOL_TEST_ENV");
    assert!(exists);
    assert_eq!(optional_value.unwrap(), "true");

    let mut value = envmnt::is_or("BOOL_TEST_ENV", false);
    assert!(value);

    envmnt::remove("BOOL_TEST_ENV");
    value = envmnt::is_or("BOOL_TEST_ENV", true);
    assert!(value);
}

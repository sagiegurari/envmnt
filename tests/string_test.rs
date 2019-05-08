extern crate envmnt;

#[test]
fn get() {
    let mut exists = envmnt::exists("STRING_TEST_ENV");
    assert!(!exists);

    envmnt::set("STRING_TEST_ENV", "1");
    exists = envmnt::exists("STRING_TEST_ENV");
    assert!(exists);

    let optional_value = envmnt::get_set("STRING_TEST_ENV", "2");
    exists = envmnt::exists("STRING_TEST_ENV");
    assert!(exists);
    assert_eq!(optional_value.unwrap(), "1");

    let mut value = envmnt::get_or("STRING_TEST_ENV", "3");
    assert_eq!(value, "2");

    envmnt::remove("STRING_TEST_ENV");
    value = envmnt::get_or("STRING_TEST_ENV", "3");
    assert_eq!(value, "3");
}

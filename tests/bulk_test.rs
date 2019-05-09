extern crate envmnt;

#[test]
fn get() {
    envmnt::set("BULK_TEST_ENV1", "1");
    envmnt::set("BULK_TEST_ENV2", "2");

    let mut exists = envmnt::is_all_exists(&vec!["BULK_TEST_ENV1", "BULK_TEST_ENV2"]);
    assert!(exists);

    exists = envmnt::is_any_exists(&vec!["BULK_TEST_ENV1", "BULK_TEST_ENV2"]);
    assert!(exists);

    exists = envmnt::is_all_exists(&vec!["BULK_TEST_ENV1", "BULK_TEST_ENV2", "BULK_TEST_ENV3"]);
    assert!(!exists);

    exists = envmnt::is_any_exists(&vec!["BULK_TEST_ENV1", "BULK_TEST_ENV2", "BULK_TEST_ENV3"]);
    assert!(exists);
}

extern crate envmnt;

use envmnt::{ExpandOptions, ExpansionType};

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

    let was_env_set =
        envmnt::set_optional("STRING_TEST_SET_OPTIONAL_SOME", &Some("OPTIONAL VALUE"));
    assert!(was_env_set);
    assert!(envmnt::is_equal(
        "STRING_TEST_SET_OPTIONAL_SOME",
        "OPTIONAL VALUE"
    ));

    envmnt::set("MY_ENV_VAR2", "SOME VALUE2");

    let value = envmnt::get_any(&vec!["MY_ENV_VAR1", "MY_ENV_VAR2"], "default");
    assert!(!envmnt::exists("MY_ENV_VAR1"));
    assert!(envmnt::exists("MY_ENV_VAR2"));
    assert_eq!(value, "SOME VALUE2");

    envmnt::set("MY_ENV", "my expanded value");
    let mut options = ExpandOptions::new();
    options.expansion_type = Some(ExpansionType::Unix);
    let value = envmnt::expand("Env: MY_ENV value is: ${MY_ENV}", Some(options));
    assert_eq!("Env: MY_ENV value is: my expanded value", &value);
}

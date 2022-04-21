use envmnt;
use indexmap::IndexMap;

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

    envmnt::remove_all(&vec!["BULK_TEST_ENV1", "BULK_TEST_ENV2"]);
    exists = envmnt::is_any_exists(&vec!["BULK_TEST_ENV1", "BULK_TEST_ENV2"]);
    assert!(!exists);

    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert("BULK_TEST_ENV3".to_string(), "3".to_string());
    env.insert("BULK_TEST_ENV4".to_string(), "4".to_string());

    envmnt::set_all(&env);

    assert!(envmnt::is_equal("BULK_TEST_ENV3", "3"));
    assert!(envmnt::is_equal("BULK_TEST_ENV4", "4"));

    let eval_env = |key: String, value: String| {
        let mut updated_key = String::from("KEY-");
        updated_key.push_str(&key);
        let mut updated_value = String::from("VALUE-");
        updated_value.push_str(&value);
        Some((updated_key, updated_value))
    };

    env = IndexMap::new();
    env.insert("BULK_TEST_ENV5".to_string(), "5".to_string());
    env.insert("BULK_TEST_ENV6".to_string(), "6".to_string());

    envmnt::evaluate_and_set_all(&env, eval_env);

    assert!(envmnt::is_equal("KEY-BULK_TEST_ENV5", "VALUE-5"));
    assert!(envmnt::is_equal("KEY-BULK_TEST_ENV6", "VALUE-6"));
}

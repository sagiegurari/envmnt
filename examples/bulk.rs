extern crate envmnt;
extern crate indexmap;

use indexmap::IndexMap;

fn main() {
    let mut env: IndexMap<String, String> = IndexMap::new();
    env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());

    envmnt::set_all(&env);

    let value = envmnt::get_or_panic("ENV_VAR1");
    println!("Value Is: {}", &value);

    let mut found = envmnt::is_any_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("Any Found: {}", &found);

    found = envmnt::is_all_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("All Found: {}", &found);

    envmnt::remove_all(&vec!["ENV_VAR1", "ENV_VAR2"]);

    found = envmnt::is_any_exists(&vec!["ENV_VAR1", "ENV_VAR2"]);

    println!("Any Found: {}", &found);

    env = IndexMap::new();
    env.insert("ENV_VAR1".to_string(), "MY VALUE".to_string());
    env.insert("ENV_VAR2".to_string(), "MY VALUE2".to_string());

    let eval_env = |value: String| {
        let mut buffer = String::from("VALUE-");
        buffer.push_str(&value);
        buffer
    };

    envmnt::evaluate_and_set_all(&env, eval_env);

    let value = envmnt::get_or_panic("ENV_VAR1");
    println!("Value Is: {}", &value);
}

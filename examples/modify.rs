extern crate envmnt;

use envmnt::ExpansionType;

fn main() {
    if !envmnt::exists("MY_ENV_VAR") {
        envmnt::set("MY_ENV_VAR", "SOME VALUE");
    }

    let mut value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("Env Value: {}", &value);

    value = envmnt::get_or_panic("MY_ENV_VAR");
    println!("Env Value: {}", &value);

    let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");

    let value = envmnt::get_or("MY_ENV_VAR", "DEFAULT_VALUE");
    println!("New Env Value: {}", &value);
    println!("Previous Env Value: {:?}", &pre_value);

    let var_was_set = envmnt::set_optional("MY_ENV_VAR", &Some("OPTIONAL VALUE"));
    println!("Env Was Modified: {}", var_was_set);

    let all_vars = envmnt::vars(); // returned as Vec<(String, String)>

    for (key, value) in all_vars {
        println!("{}: {}", key, value);
    }

    envmnt::set("MY_ENV_VAR2", "SOME VALUE2");

    let value = envmnt::get_any(&vec!["MY_ENV_VAR1", "MY_ENV_VAR2"], "default");
    println!("MY_ENV_VAR1 exists: {}", envmnt::exists("MY_ENV_VAR1"));
    println!("MY_ENV_VAR2 exists: {}", envmnt::exists("MY_ENV_VAR2"));
    println!("Found value: {}", value);

    let value = envmnt::expand("Env: MY_ENV value is: ${MY_ENV}", ExpansionType::Unix);
    println!("Expanded: {}", &value);
}

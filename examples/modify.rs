extern crate envmnt;

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

    let all_vars = envmnt::vars(); // returned as Vec<(String, String)>

    for (key, value) in all_vars {
        println!("{}: {}", key, value);
    }
}

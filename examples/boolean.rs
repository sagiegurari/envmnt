extern crate envmnt;

fn main() {
    envmnt::set_bool("FLAG_VAR", true);
    let flag_value = envmnt::is_or("FLAG_VAR", false);
    println!("Bool Flag: {}", &flag_value);

    let pre_value = envmnt::get_set("MY_ENV_VAR", "SOME NEW VALUE");
    println!("Pre Value Exists: {}", &pre_value.is_some());

    envmnt::set("MY_ENV_VAR", "SOME VALUE");
    let same = envmnt::is_equal("MY_ENV_VAR", "SOME VALUE");
    println!("Value Is Same: {}", &same);
}

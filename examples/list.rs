extern crate envmnt;

fn main() {
    envmnt::set_list(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );

    let mut values = envmnt::get_list("LIST_TEST_ENV").unwrap();
    println!("List Values: {:?}", values);

    let mut same = envmnt::is_equal("LIST_TEST_ENV", "1;2;3");
    println!("Same: {}", same);

    envmnt::set_list_with_separator(
        "LIST_TEST_ENV",
        &vec!["1".to_string(), "2".to_string(), "3".to_string()],
        ",",
    );

    values = envmnt::get_list_with_separator("LIST_TEST_ENV", ",").unwrap();
    println!("List Values: {:?}", values);

    same = envmnt::is_equal("LIST_TEST_ENV", "1,2,3");
    println!("Same: {}", same);
}

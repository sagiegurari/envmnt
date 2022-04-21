use envmnt;

fn main() {
    let mut output = envmnt::load_file("./src/test/var.env");
    assert!(output.is_ok());

    let eval_env = |key: String, value: String| {
        let mut updated_key = String::from("KEY-");
        updated_key.push_str(&key);
        let mut updated_value = String::from("VALUE-");
        updated_value.push_str(&value);
        Some((updated_key, updated_value))
    };

    output = envmnt::evaluate_and_load_file("./src/test/var.env", eval_env);
    assert!(output.is_ok());
}

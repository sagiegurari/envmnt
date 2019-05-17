extern crate envmnt;

fn main() {
    let mut output = envmnt::load_file("./src/test/var.env");
    assert!(output.is_ok());

    let eval_env = |value: String| {
        let mut buffer = String::from("PREFIX-");
        buffer.push_str(&value);
        buffer
    };

    output = envmnt::evaluate_and_load_file("./src/test/var.env", eval_env);
    assert!(output.is_ok());
}

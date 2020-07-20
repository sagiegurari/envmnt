use envmnt;

fn main() {
    // all numeric data types: u8/i8/u16/i16/u32/i32/u64/i64/u128/i128/f32/f64/isize/usize
    // are supported by specific set/get functions.
    envmnt::set_u8("U8_TEST_ENV", 50);
    let value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
    println!("u8 value: {}", value_u8);

    envmnt::set_isize("ISIZE_TEST_ENV", -50);
    let value_isize = envmnt::get_isize("ISIZE_TEST_ENV", 5);
    println!("isize value: {}", value_isize);
}

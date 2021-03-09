use envmnt;

fn main() {
    // all numeric data types: u8/i8/u16/i16/u32/i32/u64/i64/u128/i128/f32/f64/isize/usize
    // are supported by specific set/get functions.
    // get_parse can support parsing various data types beyond the simple numeric getters.
    envmnt::set_u8("U8_TEST_ENV", 50);
    let mut value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
    assert_eq!(value_u8, 50);
    value_u8 = envmnt::get_parse_or("U8_TEST_ENV", 5).unwrap();
    assert_eq!(value_u8, 50);

    envmnt::set_isize("ISIZE_TEST_ENV", -50);
    let mut value_isize = envmnt::get_isize("ISIZE_TEST_ENV", 5);
    assert_eq!(value_isize, -50);
    value_isize = envmnt::get_parse("ISIZE_TEST_ENV").unwrap();
    assert_eq!(value_isize, -50);

    // increment/decrement values
    value_isize = envmnt::increment("U8_TEST_ENV");
    assert_eq!(value_isize, 51);
    value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
    assert_eq!(value_u8, 51);
    value_isize = envmnt::decrement("U8_TEST_ENV");
    assert_eq!(value_isize, 50);
    value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
    assert_eq!(value_u8, 50);
}

use envmnt;

#[test]
fn numeric() {
    envmnt::set_u8("U8_TEST_ENV", 50);
    let mut value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
    assert_eq!(value_u8, 50);

    envmnt::set_isize("ISIZE_TEST_ENV", -50);
    let mut value_isize = envmnt::get_isize("ISIZE_TEST_ENV", 5);
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

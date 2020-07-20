use envmnt;

#[test]
fn numeric() {
    envmnt::set_u8("U8_TEST_ENV", 50);
    let value_u8 = envmnt::get_u8("U8_TEST_ENV", 5);
    assert_eq!(value_u8, 50);

    envmnt::set_isize("ISIZE_TEST_ENV", -50);
    let value_isize = envmnt::get_isize("ISIZE_TEST_ENV", 5);
    assert_eq!(value_isize, -50);
}

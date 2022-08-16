use crate::Checkpoint;
use serial_test::serial;

#[test]
#[serial]
fn create() {
    crate::remove("VAR");

    crate::set("VAR", "1");

    let chk = Checkpoint::new();

    crate::remove("VAR");

    chk.restore();

    assert!(crate::is_equal("VAR", "1"));
}

#[test]
#[serial]
fn create_exclude() {
    crate::remove("VAR1");
    crate::remove("VAR2");

    crate::set("VAR1", "1");
    crate::set("VAR2", "2");

    let mut chk = Checkpoint::new();
    chk.exclude("VAR1");

    crate::remove("VAR1");
    crate::remove("VAR2");

    chk.restore();

    assert!(!crate::exists("VAR1"));
    assert_eq!(crate::get_or_panic("VAR2"), "2".to_owned())
}

#[test]
#[serial]
fn remove() {
    crate::remove("VAR");

    let chk = Checkpoint::new();

    crate::set("VAR", "1");

    chk.restore();

    assert!(!crate::exists("VAR"));
}

#[test]
#[serial]
fn remove_exclude() {
    crate::remove("VAR1");
    crate::remove("VAR2");

    let mut chk = Checkpoint::new();
    chk.exclude("VAR1");

    crate::set("VAR1", "1");
    crate::set("VAR2", "2");

    chk.restore();

    assert_eq!(crate::get_or_panic("VAR1"), "1".to_owned());
    assert!(!crate::exists("VAR2"));
}

#[test]
#[serial]
fn modify() {
    crate::remove("VAR");

    crate::set("VAR", "1");

    let chk = Checkpoint::new();

    crate::set("VAR", "2");

    chk.restore();

    assert_eq!(crate::get_or_panic("VAR"), "1".to_owned())
}

#[test]
#[serial]
fn modify_exclude() {
    crate::remove("VAR1");
    crate::remove("VAR2");

    crate::set("VAR1", "1");
    crate::set("VAR2", "2");

    let mut chk = Checkpoint::new();
    chk.exclude("VAR1");

    crate::set("VAR1", "3");
    crate::set("VAR2", "4");

    chk.restore();

    assert_eq!(crate::get_or_panic("VAR1"), "3".to_owned());
    assert_eq!(crate::get_or_panic("VAR2"), "2".to_owned())
}

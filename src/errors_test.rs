use super::*;

#[test]
fn is_file_not_found_true() {
    let error = EnvmntError {
        kind: ErrorKind::FileNotFound("test"),
    };

    assert!(error.is_file_not_found());
}

#[test]
fn is_file_not_found_false() {
    let error = EnvmntError {
        kind: ErrorKind::FileOpen("test"),
    };

    assert!(!error.is_file_not_found());
}

#[test]
fn is_file_open_true() {
    let error = EnvmntError {
        kind: ErrorKind::FileOpen("test"),
    };

    assert!(error.is_file_open());
}

#[test]
fn is_file_open_false() {
    let error = EnvmntError {
        kind: ErrorKind::FileNotFound("test"),
    };

    assert!(!error.is_file_open());
}

#[test]
fn description_file_not_found() {
    let error = EnvmntError {
        kind: ErrorKind::FileNotFound("test"),
    };

    assert_eq!(error.description(), "test");
}

#[test]
fn description_file_open() {
    let error = EnvmntError {
        kind: ErrorKind::FileOpen("test"),
    };

    assert_eq!(error.description(), "test");
}

#[test]
fn format_file_not_found() {
    let error = EnvmntError {
        kind: ErrorKind::FileNotFound("test"),
    };

    // ensure compiles
    println!("{}", &error);
}

#[test]
fn format_file_open() {
    let error = EnvmntError {
        kind: ErrorKind::FileOpen("test"),
    };

    // ensure compiles
    println!("{}", &error);
}

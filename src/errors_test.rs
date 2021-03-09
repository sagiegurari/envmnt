use super::*;
use fsio::error::FsIOError;

#[test]
fn format_file_open() {
    let error = EnvmntError::ReadFile(
        "Unable to read file.",
        FsIOError::PathAlreadyExists("test".to_string()),
    );

    let error_string = error.to_string();
    assert!(error_string.contains("Unable to read file."));
    assert!(error_string.contains("test"));
}

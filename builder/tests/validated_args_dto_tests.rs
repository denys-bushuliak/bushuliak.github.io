use std::path::PathBuf;

use builder::{Args, ArgumentsValidationError, ValidatedArgsDto};

fn temp_dir(suffix: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("builder_test_{}_{}", suffix, uuid()));
    std::fs::create_dir_all(&dir).expect("failed to create temp dir");
    dir
}

fn uuid() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    format!("{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

#[test]
fn valid_input_directory_returns_validated_args() {
    let temp = temp_dir("valid_input");

    let args = Args {
        r#in: temp.clone(),
        out: PathBuf::from("/tmp/output"),
    };

    let result: Result<ValidatedArgsDto, ArgumentsValidationError> = args.try_into();

    assert!(result.is_ok());
    let validated = result.unwrap();
    assert_eq!(validated.input_directory, temp);
}

#[test]
fn non_existent_input_directory_returns_error() {
    let args = Args {
        r#in: PathBuf::from("/tmp/nonexistent_builder_dir_12345"),
        out: PathBuf::from("/tmp/output"),
    };

    let result: Result<ValidatedArgsDto, ArgumentsValidationError> = args.try_into();

    assert!(result.is_err());
    match result.unwrap_err() {
        ArgumentsValidationError::InputDirectoryDoesNotExist(path) => {
            assert_eq!(path, PathBuf::from("/tmp/nonexistent_builder_dir_12345"));
        }
        _ => panic!("Expected InputDirectoryDoesNotExist, but got a different error"),
    }
}

#[test]
fn input_path_is_file_returns_error() {
    let temp = temp_dir("file_input");

    // Create a file at the path instead of a directory
    let file_path = temp.join("test.txt");
    std::fs::write(&file_path, "content").expect("failed to write test file");

    let args = Args {
        r#in: file_path.clone(),
        out: PathBuf::from("/tmp/output"),
    };

    let result: Result<ValidatedArgsDto, ArgumentsValidationError> = args.try_into();

    assert!(result.is_err());
    match result.unwrap_err() {
        ArgumentsValidationError::InputShouldBeDirectory(path) => {
            assert_eq!(path, file_path);
        }
        _ => panic!("Expected InputShouldBeDirectory, but got a different error"),
    }

    // Cleanup
    std::fs::remove_dir_all(temp).ok();
}

#[test]
fn output_directory_is_preserved_as_provided() {
    let temp = temp_dir("output_preservation");

    let output_path = PathBuf::from("/tmp/custom_output_dir");
    let args = Args {
        r#in: temp.clone(),
        out: output_path.clone(),
    };

    let result: Result<ValidatedArgsDto, ArgumentsValidationError> = args.try_into();

    assert!(result.is_ok());
    let validated = result.unwrap();
    assert_eq!(validated.output_directory, output_path);

    std::fs::remove_dir_all(temp).ok();
}

#[test]
fn validation_error_display_format() {
    let path = PathBuf::from("/some/path");

    match ArgumentsValidationError::InputDirectoryDoesNotExist(path.clone()) {
        e => assert!(format!("{}", e).contains("does not exist")),
    }

    match ArgumentsValidationError::InputShouldBeDirectory(path) {
        e => assert!(format!("{}", e).contains("not a directory")),
    }
}

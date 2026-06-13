use std::{error::Error, fmt::Display, path::PathBuf};

use super::args_dto::Args;

#[derive(Debug)]
pub struct ValidatedArgsDto {
    /// Source folder with markdowns
    pub input_directory: PathBuf,

    /// Destination folder for prepared html
    pub output_directory: PathBuf,
}
impl TryFrom<Args> for ValidatedArgsDto {
    type Error = ArgumentsValidationError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        let input_directory = if value.r#in.exists() {
            if value.r#in.is_dir() {
                value.r#in
            } else {
                return Err(ArgumentsValidationError::InputShouldBeDirectory(value.r#in));
            }
        } else {
            return Err(ArgumentsValidationError::InputDirectoryDoesNotExist(
                value.r#in,
            ));
        };

        let output_directory = value.out;

        Ok(Self {
            input_directory,
            output_directory,
        })
    }
}
#[derive(Debug)]
pub enum ArgumentsValidationError {
    InputDirectoryDoesNotExist(PathBuf),
    InputShouldBeDirectory(PathBuf),
}

impl Display for ArgumentsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentsValidationError::InputDirectoryDoesNotExist(p) => {
                write!(f, "Input directory does not exist: {p:?}")
            }
            ArgumentsValidationError::InputShouldBeDirectory(p) => {
                write!(f, "Input path is not a directory: {p:?}")
            }
        }
    }
}

impl Error for ArgumentsValidationError {}

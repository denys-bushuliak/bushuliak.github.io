pub mod entities;
pub mod functions;

pub use entities::args_dto::Args;
pub use entities::html_file::HtmlFile;
pub use entities::markdown_file::MarkdownFile;
pub use entities::validated_args_dto::{ArgumentsValidationError, ValidatedArgsDto};

use std::error::Error;
use std::fs::File;
use std::io::Read;
use functions::{convert_to_html, read_directory, save_to_disk};

pub const LAYOUT_FILE_PATH: &str = "template.html";
pub const CONTENT_PLACEHOLDER: &str = "<!--REPLACE_ME_BY_CONTENT-->";

pub fn run(args: ValidatedArgsDto) -> Result<(), Box<dyn Error>> {
    let mut layout_file = String::new();
    File::open(LAYOUT_FILE_PATH)?.read_to_string(&mut layout_file)?;

    let input_dir = args.input_directory.clone();
    let it = read_directory(args.input_directory)?;
    it.map(MarkdownFile::from)
        .map(convert_to_html(
            &args.output_directory,
            &layout_file,
            CONTENT_PLACEHOLDER,
            &input_dir,
        ))
        .try_for_each(save_to_disk)?;
    Ok(())
}

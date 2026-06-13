use std::{
    error::Error,
    fmt::Display,
    fs::{self, DirEntry},
    io::Write,
    path::{Path, PathBuf},
};

use crate::MarkdownFile;
use crate::entities::html_file::HtmlFile;

pub fn read_directory(
    dir_path: PathBuf,
) -> Result<impl Iterator<Item = (PathBuf, String)>, std::io::Error> {
    let entries: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(|e| e.ok())
        .flat_map(recur_read_files)
        .collect();
    Ok(entries.into_iter())
}

fn recur_read_files(entry: DirEntry) -> Vec<(PathBuf, String)> {
    let path = entry.path();

    // Skip hidden files and directories (macOS, git, etc.)
    if entry.file_name().to_string_lossy().starts_with('.') {
        return Vec::new();
    }

    if path.is_dir() {
        read_directory(path)
            .map(|it| it.collect())
            .unwrap_or_default()
    } else if path.extension().is_some_and(|ext| ext == "md") {
        fs::read_to_string(&path)
            .map(|content| vec![(path, content)])
            .unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn convert_to_html(
    output_directory: &Path,
    layout_html_file: &str,
    placeholder: &str,
    input_directory: &Path,
) -> impl Fn(MarkdownFile) -> HtmlFile {
    move |markdown_file: MarkdownFile| {
        // Compute relative path from input dir, preserving subdirectory structure
        let relative = markdown_file
            .path
            .strip_prefix(input_directory)
            .unwrap_or(&markdown_file.path);

        let mut path_to_save = output_directory.join(relative);
        path_to_save.set_extension("html");

        let parser = pulldown_cmark::Parser::new(&markdown_file.content);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);

        let html_output = layout_html_file.replace(placeholder, &html_output);

        HtmlFile::new(path_to_save, html_output)
    }
}

#[derive(Debug)]
pub enum SaveToDiskError {
    ErrorOnSavingFile(std::io::Error),
    CanNotTakeParentDirectoryFrom(PathBuf),
}

impl Display for SaveToDiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SaveToDiskError::ErrorOnSavingFile(e) => write!(f, "IO error: {e}"),
            SaveToDiskError::CanNotTakeParentDirectoryFrom(p) => {
                write!(f, "Cannot determine parent directory for: {p:?}")
            }
        }
    }
}

impl From<std::io::Error> for SaveToDiskError {
    fn from(value: std::io::Error) -> Self {
        SaveToDiskError::ErrorOnSavingFile(value)
    }
}

impl Error for SaveToDiskError {}

pub fn save_to_disk(html_file: HtmlFile) -> Result<(), SaveToDiskError> {
    let dir = html_file.path_to_save.parent().ok_or_else(|| {
        SaveToDiskError::CanNotTakeParentDirectoryFrom(html_file.path_to_save.clone())
    })?;

    fs::create_dir_all(dir)?;
    let mut file = std::fs::File::create(&html_file.path_to_save)?;
    file.write_all(html_file.content.as_bytes())?;

    Ok(())
}

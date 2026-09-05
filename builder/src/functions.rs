use std::{
    error::Error,
    fmt::Display,
    fs::{self, DirEntry},
    io::Write,
    path::{Path, PathBuf},
};

use crate::MarkdownFile;
use crate::PageMeta;
use crate::CONTENT_PLACEHOLDER;
use crate::DESCRIPTION_PLACEHOLDER;
use crate::TITLE_PLACEHOLDER;
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

        let html_output = render_markdown(&markdown_file.content);

        let stem = markdown_file
            .path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("");
        let meta = PageMeta::from_file_stem(stem);

        let html_output = layout_html_file
            .replace(placeholder, &html_output)
            .replace(TITLE_PLACEHOLDER, &meta.title)
            .replace(DESCRIPTION_PLACEHOLDER, &meta.description);

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

pub fn render_markdown(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

/// Markdown sources start with a per-page active-nav highlight; it must not
/// leak into a document that combines several sources.
fn strip_leading_style_block(content: &str) -> &str {
    let trimmed = content.trim_start();
    match trimmed.strip_prefix("<style>") {
        Some(rest) => rest
            .split_once("</style>")
            .map(|(_, after)| after.trim_start())
            .unwrap_or(trimmed),
        None => content,
    }
}

/// The sources combined into the single printable résumé page.
const RESUME_PARTS: &[(&str, &str)] = &[
    ("about.md", "About Me"),
    ("projects.md", "Projects"),
    ("skills.md", "Skills"),
    ("recommendation_letters.md", "Recommendation Letters"),
];

/// Builds `resume.html`: all résumé-relevant pages combined into one
/// document, so the whole site can be printed in a single A4 job.
pub fn build_resume(
    output_directory: &Path,
    input_directory: &Path,
    layout_html_file: &str,
) -> Result<HtmlFile, Box<dyn Error>> {
    let mut content = String::from(
        "<style>\n    #resume-link{\n        color: var(--active-nav-link-color) !important;\n    }\n</style>\n",
    );

    for (file, title) in RESUME_PARTS {
        let part_path = input_directory.join(file);
        let part = fs::read_to_string(&part_path).map_err(|e| {
            format!("Cannot read resume part {}: {e}", part_path.display())
        })?;
        let part_html = render_markdown(&strip_leading_style_block(&part));
        content.push_str(&format!(
            "<section class=\"resume-part\">\n<h1>{title}</h1>\n{part_html}</section>\n"
        ));
    }

    let meta = PageMeta::from_file_stem("resume");
    let html_output = layout_html_file
        .replace(CONTENT_PLACEHOLDER, &content)
        .replace(TITLE_PLACEHOLDER, &meta.title)
        .replace(DESCRIPTION_PLACEHOLDER, &meta.description);

    Ok(HtmlFile::new(output_directory.join("resume.html"), html_output))
}

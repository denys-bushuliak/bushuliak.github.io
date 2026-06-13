use std::path::PathBuf;

#[derive(Debug)]
pub struct HtmlFile {
    pub path_to_save: PathBuf,
    pub content: String,
}

impl HtmlFile {
    pub fn new(path_to_save: PathBuf, content: String) -> Self {
        Self {
            path_to_save,
            content,
        }
    }
}

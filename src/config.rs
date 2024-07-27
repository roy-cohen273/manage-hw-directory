use formatx::formatx;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct Config {
    downloads_dir: Option<Box<Path>>,
    subjects_dir: Box<Path>,
    max_hw_dirs: usize,
    lyx_template_file: Option<Box<Path>>,
    hw_dir_format: Box<str>,
    questions_filename_format: Box<str>,
    lyx_filename_format: Box<str>,
}

impl Config {
    pub fn downloads_dir(&self) -> Option<&Path> {
        self.downloads_dir.as_deref()
    }

    pub fn subjects_dir(&self) -> &Path {
        &self.subjects_dir
    }

    pub fn max_hw_dirs(&self) -> usize {
        self.max_hw_dirs
    }

    pub fn lyx_template_file(&self) -> Option<&Path> {
        self.lyx_template_file.as_deref()
    }

    pub fn hw_dir(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.hw_dir_format.to_owned(), num = num)
    }

    pub fn questions_filename(&self, num: usize, original: &str) -> Result<String, formatx::Error> {
        formatx!(
            self.questions_filename_format.to_owned(),
            num = num,
            original = original,
        )
    }

    pub fn lyx_filename(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.lyx_filename_format.to_owned(), num = num)
    }
}

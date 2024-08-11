use formatx::formatx;
use serde::Deserialize;
use std::path::Path;

mod lyx_file_config;
pub mod open_config;
mod questions_file_config;

use lyx_file_config::LyxFileConfig;
use questions_file_config::QuestionsFileConfig;

#[derive(Deserialize)]
pub struct Config {
    // mandatory:
    subjects_dir: Box<Path>,
    hw_dir_format: Box<str>,
    max_hw_dirs: usize,
    open_after_creation: bool,

    // questions file:
    questions_file: Option<QuestionsFileConfig>,

    // LyX file:
    lyx_file: Option<LyxFileConfig>,
}

impl Config {
    pub fn subjects_dir(&self) -> &Path {
        &self.subjects_dir
    }

    pub fn hw_dir(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.hw_dir_format.to_owned(), num = num)
    }

    pub fn max_hw_dirs(&self) -> usize {
        self.max_hw_dirs
    }

    pub fn questions_file_config(&self) -> Option<&QuestionsFileConfig> {
        self.questions_file.as_ref()
    }

    pub fn lyx_file_config(&self) -> Option<&LyxFileConfig> {
        self.lyx_file.as_ref()
    }

    pub fn open_after_creation(&self) -> bool {
        self.open_after_creation
    }
}

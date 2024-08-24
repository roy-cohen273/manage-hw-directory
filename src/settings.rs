use config::{Config, File, FileFormat, Source};
use formatx::formatx;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod lyx_file_settings;
pub mod open_settings;
mod questions_file_settings;
// mod subject_config;

use lyx_file_settings::LyxFileSettings;
use questions_file_settings::QuestionsFileSettings;

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    // mandatory:
    subjects_dir: Box<Path>,
    hw_dir_format: Box<str>,
    max_hw_dirs: usize,
    open_after_creation: bool,

    // optional
    subject_settings_filename: Option<Box<str>>,
    hebrew_name: Option<Box<str>>,

    // questions file:
    questions_file: Option<QuestionsFileSettings>,

    // LyX file:
    lyx_file: Option<LyxFileSettings>,
}

impl Settings {
    pub fn new(
        sources: impl IntoIterator<Item: Source + Send + Sync + 'static>,
    ) -> anyhow::Result<Self> {
        let mut builder = Config::builder();
        for source in sources {
            builder = builder.add_source(source);
        }
        Ok(builder.build()?.try_deserialize()?)
    }

    pub fn update(
        &self,
        sources: impl IntoIterator<Item: Source + Send + Sync + 'static>,
    ) -> anyhow::Result<Self> {
        let mut builder = Config::builder().add_source(File::from_str(
            &serde_json::to_string(self)?,
            FileFormat::Json,
        ));

        for source in sources {
            builder = builder.add_source(source);
        }

        Ok(builder.build()?.try_deserialize()?)
    }

    pub fn subjects_dir(&self) -> &Path {
        &self.subjects_dir
    }

    pub fn hw_dir(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.hw_dir_format.to_owned(), num = num)
    }

    pub fn max_hw_dirs(&self) -> usize {
        self.max_hw_dirs
    }

    pub fn open_after_creation(&self) -> bool {
        self.open_after_creation
    }

    pub fn subject_settings_filename(&self) -> Option<&str> {
        self.subject_settings_filename.as_deref()
    }

    pub fn hebrew_name(&self) -> &str {
        self.hebrew_name.as_deref().unwrap_or("")
    }

    pub fn questions_file_settings(&self) -> Option<&QuestionsFileSettings> {
        self.questions_file.as_ref()
    }

    pub fn lyx_file_settings(&self) -> Option<&LyxFileSettings> {
        self.lyx_file.as_ref()
    }
}

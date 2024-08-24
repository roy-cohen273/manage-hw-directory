use super::open_settings::{Formattable, OpenSettings};
use formatx::formatx;
use serde::{Deserialize, Serialize};
use std::path::{self, Path};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionsFileSettings {
    downloads_dir: Box<Path>,
    questions_filename_format: Box<str>,

    open: Option<OpenSettings<QuestionsFile>>,
}

impl QuestionsFileSettings {
    pub fn downloads_dir(&self) -> &Path {
        &self.downloads_dir
    }

    pub fn questions_filename(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.questions_filename_format.to_owned(), num = num)
    }

    pub fn open_settings(&self) -> Option<&OpenSettings<QuestionsFile>> {
        self.open.as_ref()
    }
}

pub struct QuestionsFile;
impl Formattable for QuestionsFile {
    type Params = Path;

    fn format(s: String, questions_file: &Path) -> anyhow::Result<String> {
        let absolute_questions_file = path::absolute(questions_file)?;
        let questions_file = absolute_questions_file
            .to_str()
            .ok_or(anyhow::anyhow!("cannot convert questions file to string"))?;

        formatx!(s, questions_file = questions_file).map_err(Into::into)
    }
}

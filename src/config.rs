use anyhow::anyhow;
use formatx::formatx;
use serde::Deserialize;
use std::marker::PhantomData;
use std::path::{self, Path};

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

#[derive(Deserialize)]
pub struct QuestionsFileConfig {
    downloads_dir: Box<Path>,
    questions_filename_format: Box<str>,

    open: Option<OpenConfig<QuestionsFile>>,
}

#[derive(Deserialize)]
pub struct LyxFileConfig {
    lyx_template_file: Option<Box<Path>>,
    lyx_filename_format: Box<str>,
    replacements: Box<[LyxReplacementConfig]>,

    open: Option<OpenConfig<LyxFile>>,
}

#[derive(Deserialize)]
pub struct LyxReplacementConfig {
    from: Box<str>,
    to_format: Box<str>,
    count: Option<usize>,
}

pub trait Formattable {
    type Params: ?Sized;

    fn format(s: String, params: &Self::Params) -> anyhow::Result<String>;
}

#[derive(Deserialize)]
pub struct OpenConfig<T: Formattable> {
    binary: Box<str>,
    args_format: Box<[Box<str>]>,

    #[serde(skip)]
    _phantom: PhantomData<T>,
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

pub struct LyxFile;
impl Formattable for LyxFile {
    type Params = Path;

    fn format(s: String, lyx_file: &Path) -> anyhow::Result<String> {
        let absolute_lyx_file = path::absolute(lyx_file)?;
        let lyx_file = absolute_lyx_file
            .to_str()
            .ok_or(anyhow!("cannot convert LyX file to string"))?;

        formatx!(s, lyx_file = lyx_file).map_err(Into::into)
    }
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

impl QuestionsFileConfig {
    pub fn downloads_dir(&self) -> &Path {
        &self.downloads_dir
    }

    pub fn questions_filename(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.questions_filename_format.to_owned(), num = num)
    }

    pub fn open_config(&self) -> Option<&OpenConfig<QuestionsFile>> {
        self.open.as_ref()
    }
}

impl LyxFileConfig {
    pub fn lyx_template_file(&self) -> Option<&Path> {
        self.lyx_template_file.as_deref()
    }

    pub fn lyx_filename(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.lyx_filename_format.to_owned(), num = num)
    }

    pub fn replacements(&self) -> &[LyxReplacementConfig] {
        &self.replacements
    }

    pub fn open_config(&self) -> Option<&OpenConfig<LyxFile>> {
        self.open.as_ref()
    }
}

impl LyxReplacementConfig {
    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self, num: usize) -> Result<String, formatx::Error> {
        formatx!(self.to_format.to_owned(), num = num)
    }

    pub fn count(&self) -> Option<usize> {
        self.count
    }
}

impl<T: Formattable> OpenConfig<T> {
    pub fn binary(&self) -> &str {
        &self.binary
    }

    pub fn args(&self, params: &T::Params) -> anyhow::Result<impl Iterator<Item = String>> {
        self.args_format
            .iter()
            .map(|arg_format| T::format(arg_format.to_string(), params))
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
            .map(IntoIterator::into_iter)
    }
}

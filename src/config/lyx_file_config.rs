use super::open_config::{Formattable, OpenConfig};
use anyhow::anyhow;
use formatx::formatx;
use serde::Deserialize;
use std::path;
use std::path::Path;

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

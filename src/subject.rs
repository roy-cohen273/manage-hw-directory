use crate::settings::Settings;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

mod files;

#[derive(Clone)]
pub struct Subject {
    dir: PathBuf,
    name: String,
    current_hw_num: usize,
    settings: Settings,
}

impl Subject {
    pub fn path(&self) -> &Path {
        &self.dir
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn current_hw_num(&self) -> usize {
        self.current_hw_num
    }
}

impl Subject {
    pub fn from_directory(settings: &Settings, dir: PathBuf) -> anyhow::Result<Self> {
        let settings = files::update_subject_settings(settings, &dir)?;
        let name = dir
            .file_name()
            .and_then(OsStr::to_str)
            .map(str::to_owned)
            .ok_or(anyhow::anyhow!("directory has no filename"))?;
        let current_hw_num = files::get_last_hw_num(&settings, &dir)?;
        Ok(Self {
            dir,
            name,
            current_hw_num,
            settings,
        })
    }

    pub fn get_all_subjects(settings: &Settings) -> anyhow::Result<Box<[Self]>> {
        Ok(settings.subject_ordering().sort_subjects(
            settings
                .subjects_dir()
                .read_dir()?
                .filter_map(Result::ok)
                .filter_map(|dir_entry| Self::from_directory(settings, dir_entry.path()).ok()),
        ))
    }

    pub fn open_last_hw(&self) -> anyhow::Result<()> {
        files::open_last_hw_dir(&self.settings, &self.dir)
    }

    pub fn create_new_hw_dir(&mut self) -> anyhow::Result<()> {
        files::create_new_hw_dir(&self.settings, &self.dir)?;
        self.current_hw_num += 1;
        Ok(())
    }
}

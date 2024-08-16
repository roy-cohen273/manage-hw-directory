use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

use crate::settings::{
    open_settings::{Formattable, OpenSettings},
    Settings,
};

/// Create a new HW folder under the specified subject directory,
/// and move the most recently downloaded file (from the downloads directory) to there.
pub fn create_new_hw_dir(settings: &Settings, subject_dir: &Path) -> anyhow::Result<()> {
    let (num, hw_dir) = create_hw_dir(settings, subject_dir)?;
    create_questions_file(settings, num, &hw_dir)?;
    create_lyx_file(settings, num, &hw_dir)?;
    if settings.open_after_creation() {
        open_hw_dir(settings, &hw_dir, num)?;
    }
    Ok(())
}

/// Get a list of available subjects
pub fn get_subjects(settings: &Settings) -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    Ok(list_dir(settings.subjects_dir())?.filter(|path| path.is_dir()))
}

/// Open the last HW directory in the given subject.
pub fn open_last_hw_dir(settings: &Settings, subject_dir: &Path) -> anyhow::Result<()> {
    let num = get_last_hw_num(settings, subject_dir)?;
    let hw_dir = subject_dir.join(settings.hw_dir(num)?);
    open_hw_dir(settings, &hw_dir, num)?;

    Ok(())
}

fn list_dir(dir: &Path) -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    Ok(dir
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path()))
}

fn get_last_hw_num(settings: &Settings, subject_dir: &Path) -> anyhow::Result<usize> {
    // search for the next HW num
    let paths: Box<[_]> = list_dir(subject_dir)?.collect();
    let used_filenames: HashSet<_> = paths
        .iter()
        .filter_map(|path| path.file_name().and_then(|s| s.to_str()))
        .collect();
    for num in (0..=settings.max_hw_dirs()).rev() {
        let filename = settings.hw_dir(num)?;
        if used_filenames.contains(&&*filename) {
            return Ok(num);
        }
    }
    Ok(0)
}

fn create_hw_dir(settings: &Settings, subject_dir: &Path) -> anyhow::Result<(usize, PathBuf)> {
    let num = get_last_hw_num(settings, subject_dir)? + 1;
    if num > settings.max_hw_dirs() {
        anyhow::bail!("Maximum number of HW directories reached");
    }

    let hw_dir = subject_dir.join(settings.hw_dir(num)?);
    fs::create_dir(&hw_dir)?;

    Ok((num, hw_dir))
}

fn create_questions_file(settings: &Settings, num: usize, hw_dir: &Path) -> anyhow::Result<()> {
    let Some(questions_file_settings) = settings.questions_file_settings() else {
        return Ok(());
    };

    let questions_file_src = get_most_recent_download(questions_file_settings.downloads_dir())?;
    let questions_file_dest = hw_dir.join(questions_file_settings.questions_filename(num)?);

    fs::rename(questions_file_src, questions_file_dest)?;

    Ok(())
}

fn get_most_recent_download(downloads_directory: &Path) -> anyhow::Result<PathBuf> {
    let most_recent_download = downloads_directory
        .read_dir()?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let meta = entry.metadata().ok()?;
            let created = meta.created().ok()?;
            Some((entry, created))
        })
        .max_by_key(|(_entry, created)| *created)
        .ok_or(anyhow::anyhow!("downloads directory was empty"))?
        .0;

    Ok(most_recent_download.path())
}

fn create_lyx_file(settings: &Settings, num: usize, dir: &Path) -> anyhow::Result<()> {
    let Some(lyx_file_settings) = settings.lyx_file_settings() else {
        return Ok(());
    };

    let lyx_file = dir.join(lyx_file_settings.lyx_filename(num)?);

    if let Some(lyx_template) = lyx_file_settings.lyx_template_file() {
        if lyx_file_settings.replacements().is_empty() {
            // copy from LyX template file. no replacements.
            fs::copy(lyx_template, lyx_file)?;
        } else {
            // copy from LyX template file with replacements.
            let mut data = fs::read_to_string(lyx_template)?;
            for replace in lyx_file_settings.replacements() {
                let from = replace.from();
                let to = replace.to(num)?;
                data = if let Some(count) = replace.count() {
                    data.replacen(from, &to, count)
                } else {
                    data.replace(from, &to)
                }
            }
            fs::write(lyx_file, data)?;
        }
    } else {
        // create a new empty file
        fs::File::create(lyx_file)?;
    }

    Ok(())
}

fn open_hw_dir(settings: &Settings, hw_dir: &Path, num: usize) -> anyhow::Result<()> {
    open_questions_file(settings, hw_dir, num)?;
    open_lyx_file(settings, hw_dir, num)?;

    Ok(())
}

fn open_from_settings<T: Formattable>(
    open_settings: &OpenSettings<T>,
    params: &T::Params,
) -> anyhow::Result<()> {
    Command::new(open_settings.binary())
        .args(open_settings.args(params)?)
        .stdin(process::Stdio::null())
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .spawn()
        .map(|_child| ()) // ignore child process
        .map_err(Into::into)
}

fn open_questions_file(settings: &Settings, hw_dir: &Path, num: usize) -> anyhow::Result<()> {
    let Some(questions_file_settings) = settings.questions_file_settings() else {
        return Ok(());
    };
    let Some(open_settings) = questions_file_settings.open_settings() else {
        return Ok(());
    };

    let questions_file = hw_dir.join(questions_file_settings.questions_filename(num)?);

    open_from_settings(open_settings, &questions_file)?;

    Ok(())
}

fn open_lyx_file(settings: &Settings, hw_dir: &Path, num: usize) -> anyhow::Result<()> {
    let Some(lyx_file_settings) = settings.lyx_file_settings() else {
        return Ok(());
    };
    let Some(open_settings) = lyx_file_settings.open_settings() else {
        return Ok(());
    };

    let lyx_filename = hw_dir.join(lyx_file_settings.lyx_filename(num)?);

    open_from_settings(open_settings, &lyx_filename)?;

    Ok(())
}

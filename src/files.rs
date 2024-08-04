use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
    process::{self, Command},
};

use crate::config::Config;

/// Create a new HW folder under the specified subject directory,
/// and move the most recently downloaded file (from the downloads directory) to there.
pub fn do_the_thing(config: &Config, subject_dir: &Path) -> anyhow::Result<()> {
    let (num, hw_dir) = create_hw_dir(config, subject_dir)?;
    create_questions_file(config, num, &hw_dir)?;
    create_lyx_file(config, num, &hw_dir)?;
    if config.open_after_creation() {
        open_hw_dir(config, &hw_dir, num)?;
    }
    Ok(())
}

/// Get a list of available subjects
pub fn get_subjects(config: &Config) -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    Ok(list_dir(config.subjects_dir())?.filter(|path| path.is_dir()))
}

fn list_dir(dir: &Path) -> anyhow::Result<impl Iterator<Item = PathBuf>> {
    Ok(dir
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path()))
}

fn create_hw_dir(config: &Config, subject_dir: &Path) -> anyhow::Result<(usize, PathBuf)> {
    // search for the next HW num
    let paths: Box<[_]> = list_dir(subject_dir)?.collect();
    let used_filenames: HashSet<_> = paths
        .iter()
        .filter_map(|path| path.file_name().and_then(|s| s.to_str()))
        .collect();
    let num = 'num: {
        for num in (0..=config.max_hw_dirs()).rev() {
            let filename = config.hw_dir(num)?;
            if used_filenames.contains(&&*filename) {
                break 'num num;
            }
        }
        0
    } + 1;
    if num > config.max_hw_dirs() {
        anyhow::bail!("Maximum number of HW directories reached");
    }

    let hw_dir = subject_dir.join(config.hw_dir(num)?);
    fs::create_dir(&hw_dir)?;

    Ok((num, hw_dir))
}

fn create_questions_file(config: &Config, num: usize, hw_dir: &Path) -> anyhow::Result<()> {
    let Some(questions_file_config) = config.questions_file_config() else {
        return Ok(());
    };

    let questions_file_src = get_most_recent_download(questions_file_config.downloads_dir())?;
    let questions_file_dest = hw_dir.join(questions_file_config.questions_filename(num)?);

    move_file(&questions_file_src, &questions_file_dest)?;

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

fn move_file(src: &Path, dest: &Path) -> anyhow::Result<()> {
    fs::copy(src, dest)?;
    fs::remove_file(src)?;

    Ok(())
}

fn create_lyx_file(config: &Config, num: usize, dir: &Path) -> anyhow::Result<()> {
    let Some(lyx_file_config) = config.lyx_file_config() else {
        return Ok(());
    };

    let lyx_file = dir.join(lyx_file_config.lyx_filename(num)?);

    if let Some(lyx_template) = lyx_file_config.lyx_template_file() {
        if lyx_file_config.replacements().is_empty() {
            // copy from LyX template file. no replacements.
            fs::copy(lyx_template, lyx_file)?;
        } else {
            // copy from LyX template file with replacements.
            let mut data = fs::read_to_string(lyx_template)?;
            for replace in lyx_file_config.replacements() {
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

fn open_hw_dir(config: &Config, hw_dir: &Path, num: usize) -> anyhow::Result<()> {
    open_questions_file(config, hw_dir, num)?;

    Ok(())
}

fn open_questions_file(config: &Config, hw_dir: &Path, num: usize) -> anyhow::Result<()> {
    let Some(questions_file_config) = config.questions_file_config() else {
        return Ok(());
    };
    let Some(open_config) = questions_file_config.open_config() else {
        return Ok(());
    };

    let questions_file = hw_dir.join(questions_file_config.questions_filename(num)?);

    let mut command = Command::new(open_config.binary());

    command.args(open_config.args(&questions_file)?);

    command.stdin(process::Stdio::null());
    command.stdout(process::Stdio::null());
    command.stderr(process::Stdio::null());

    command.spawn()?;

    Ok(())
}

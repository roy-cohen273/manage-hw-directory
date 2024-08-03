use crate::config::Config;
use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

/// Create a new HW folder under the specified subject directory,
/// and move the most recently downloaded file (from the downloads directory) to there.
pub fn do_the_thing(config: &Config, subject_dir: &Path) -> io::Result<()> {
    let (num, hw_dir) = create_hw_dir(config, subject_dir)?;
    create_questions_file(config, num, &hw_dir)?;
    create_lyx_file(config, num, &hw_dir)?;
    Ok(())
}

/// Get a list of available subjects
pub fn get_subjects(config: &Config) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(list_dir(config.subjects_dir())?.filter(|path| path.is_dir()))
}

fn list_dir(dir: &Path) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(dir
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path()))
}

fn create_hw_dir(config: &Config, subject_dir: &Path) -> io::Result<(usize, PathBuf)> {
    // search for the next HW num
    let paths: Box<[_]> = list_dir(subject_dir)?.collect();
    let used_filenames: HashSet<_> = paths
        .iter()
        .filter_map(|path| path.file_name().and_then(|s| s.to_str()))
        .collect();
    let num = 'num: {
        for num in (0..=config.max_hw_dirs()).rev() {
            let filename = config.hw_dir(num).map_err(io::Error::other)?;
            if used_filenames.contains(&&*filename) {
                break 'num num;
            }
        }
        0
    } + 1;
    if num > config.max_hw_dirs() {
        return Err(io::Error::other("Maximum number of HW directories reached"));
    }

    let hw_dir = subject_dir.join(config.hw_dir(num).map_err(io::Error::other)?);
    fs::create_dir(&hw_dir)?;

    Ok((num, hw_dir))
}

fn create_questions_file(config: &Config, num: usize, hw_dir: &Path) -> io::Result<()> {
    let Some(downloads_dir) = config.downloads_dir() else {
        return Ok(());
    };

    let questions_file_src = get_most_recent_download(&downloads_dir)?;
    let questions_file_src_filename = questions_file_src
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or(io::Error::other("Most recent download has no filename"))?;
    let questions_file_dest_filename = config
        .questions_filename(num, questions_file_src_filename)
        .map_err(io::Error::other)?;
    let questions_file_dest = hw_dir.join(questions_file_dest_filename);

    move_file(&questions_file_src, &questions_file_dest)?;

    Ok(())
}

fn get_most_recent_download(downloads_directory: &Path) -> io::Result<PathBuf> {
    let most_recent_download = downloads_directory
        .read_dir()?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let meta = entry.metadata().ok()?;
            let created = meta.created().ok()?;
            Some((entry, created))
        })
        .max_by_key(|(_entry, created)| *created)
        .ok_or(io::Error::new(
            io::ErrorKind::NotFound,
            "downloads directory was empty",
        ))?
        .0;

    Ok(most_recent_download.path())
}

fn move_file(src: &Path, dest: &Path) -> io::Result<()> {
    fs::copy(src, dest)?;
    fs::remove_file(src)?;

    Ok(())
}

fn create_lyx_file(config: &Config, num: usize, dir: &Path) -> io::Result<()> {
    let Some(lyx_template) = config.lyx_template_file() else {
        return Ok(());
    };

    let lyx_file = dir.join(config.lyx_filename(num).map_err(io::Error::other)?);

    fs::copy(lyx_template, lyx_file)?;

    Ok(())
}

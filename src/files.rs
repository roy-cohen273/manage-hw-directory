use std::{
    fs, io,
    path::{Path, PathBuf},
};
use std::collections::HashSet;
use crate::config::{
    DOWNLOADS_DIR,
    SUBJECTS_DIR,
    MAX_HW_DIRS,
    LYX_TEMPLATE_FILE,
    get_hw_dir,
    get_questions_filname,
    get_lyx_filename,
};

/// Create a new HW folder under the specified subject directory,
/// and move the most recently downloaded file (from the downloads directory) to there.
pub fn do_the_thing(subject_dir: &Path) -> io::Result<()> {
    let (num, hw_dir) = create_hw_dir(&subject_dir)?;
    create_questions_file(num, &hw_dir)?;
    create_lyx_file(num, &hw_dir)?;
    Ok(())
}

/// Get a list of available subjects
pub fn get_subjects() -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(list_dir(Path::new(SUBJECTS_DIR))?.filter(|path| path.is_dir()))
}

fn list_dir(dir: &Path) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(dir
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path()))
}

fn create_hw_dir(subject_dir: &Path) -> io::Result<(usize, PathBuf)> {
    // search for the next HW num
    let paths: Box<[_]> = list_dir(subject_dir)?.collect();
    let used_filenames: HashSet<_> = paths.iter()
        .filter_map(|path|
            path.file_name()
                .and_then(|s| s.to_str())
        )
        .collect();
    let mut used_hw_num = (0..=MAX_HW_DIRS).rev()
        .filter(|num| {
            let filename = get_hw_dir(*num);
            used_filenames.contains(&&*filename)
        });
    let num = used_hw_num.next().unwrap_or(0) + 1;
    if num > MAX_HW_DIRS {
        return Err(io::Error::other("Maximum number of HW directories reached"));
    }

    let mut hw_dir = subject_dir.to_path_buf();
    hw_dir.push(get_hw_dir(num));
    fs::create_dir(&hw_dir)?;

    Ok((num, hw_dir))
}

fn create_questions_file(num: usize, hw_dir: &Path) -> io::Result<()> {
    let Some(downloads_dir) = DOWNLOADS_DIR else {
        return Ok(());
    };
    let downloads_dir = Path::new(downloads_dir);

    let questions_file_src = get_most_recent_download(&downloads_dir)?;
    let questions_file_src_filename = questions_file_src
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or(io::Error::other("Most recent download has no filename"))?;
    let questions_file_dest_filename = get_questions_filname(num, questions_file_src_filename);
    let mut questions_file_dest = hw_dir.to_path_buf();
    questions_file_dest.push(questions_file_dest_filename);

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
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "downloads directory was empty"))?
        .0;

    Ok(most_recent_download.path())
}

fn move_file(src: &Path, dest: &Path) -> io::Result<()> {
    fs::copy(src, dest)?;
    fs::remove_file(src)?;

    Ok(())
}

fn create_lyx_file(num: usize, dir: &Path) -> io::Result<()> {
    let Some(lyx_template) = LYX_TEMPLATE_FILE else {
        return Ok(());
    };

    let mut lyx_file = dir.to_path_buf();
    lyx_file.push(get_lyx_filename(num));

    fs::copy(lyx_template, lyx_file)?;

    Ok(())
}

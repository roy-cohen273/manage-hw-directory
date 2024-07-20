use std::{
    fs, io,
    path::{Path, PathBuf},
};

// real config:
// const DOWNLOADS: &str = "C:\\Users\\Roy Cohen\\Downloads";
// const SUBJECTS: &str = "G:\\My Drive";
// const HW_PREFIX: &str = "HW";

// testing config:
const DOWNLOADS: &str = "C:\\Users\\Roy Cohen\\Documents\\testing\\Downloads";
const SUBJECTS: &str = "C:\\Users\\Roy Cohen\\Documents\\testing\\Subjects";
const HW_PREFIX: &str = "HW";

/// Create a new HW folder under the specified subject directory,
/// and move the most recently downloaded file (from the downloads directory) to there.
pub fn do_the_thing(subject_dir: &Path) -> io::Result<()> {
    let questions_file = get_most_recent_download()?;
    let hw_dir = create_hw_dir(&subject_dir)?;
    move_file(&questions_file, &hw_dir)?;
    Ok(())
}

/// Get a list of available subjects
pub fn get_subjects() -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(list_dir(Path::new(SUBJECTS))?.filter(|path| path.is_dir()))
}

fn list_dir(dir: &Path) -> io::Result<impl Iterator<Item = PathBuf>> {
    Ok(dir
        .read_dir()?
        .filter_map(Result::ok)
        .map(|entry| entry.path()))
}

fn create_hw_dir(subject_dir: &Path) -> io::Result<PathBuf> {
    // search the directory for entries fo the format: HW#
    // where # is some number
    // choose the smallest number that would not collide
    let next_hw_num = list_dir(subject_dir)?
        .filter_map(|path| {
            let filename = path.file_name()?;
            let filename = filename.to_str()?;
            if !filename.starts_with(HW_PREFIX) {
                return None;
            }
            let num: u8 = filename[HW_PREFIX.len()..].parse().ok()?;
            Some(num)
        })
        .max()
        .unwrap_or(0)
        + 1;

    let mut hw_dir = subject_dir.to_path_buf();
    hw_dir.push(format!("HW{next_hw_num}"));
    fs::create_dir(&hw_dir)?;

    Ok(hw_dir)
}

fn get_most_recent_download() -> io::Result<PathBuf> {
    let most_recent_download = Path::new(DOWNLOADS)
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

fn move_file(file: &Path, dir: &Path) -> io::Result<()> {
    let mut dest_file = dir.to_path_buf();
    dest_file.push(
        file.file_name()
            .ok_or(io::Error::other("move_file: source file has no filename"))?,
    );

    fs::copy(file, dest_file)?;
    fs::remove_file(file)?;

    Ok(())
}

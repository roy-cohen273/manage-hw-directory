//! This file contains configuration values.
//!
//! The config values that have `_FORMAT` at the end of their name are format specifiers.
//! They are used with the [`format!`] macro. See the [`format!`] documentation for syntax.
//! Each format specifier has a list of named arguments it can use.
//! Do not use positional arguments, or named arguments that are not from that list.
//! Otherwise, you will get confusing error messages.

/// Path to the downloads directory.
/// This is the directory from which the last created file will be taken.
pub const DOWNLOADS_DIR: Option<&str> = Some(r"testing/downloads");

/// Path to the subjects directory.
/// This directory will contain a directory for each subject.
pub const SUBJECTS_DIR: &str = r"testing/subjects";

/// Path to the LyX template file.
/// This file will be copied to the newly created HW directory.
/// If this behaviour is undesirable, set this to `None`.
pub const LYX_TEMPLATE_FILE: Option<&str> = Some(r"testing/mytemplate.lyx");

/// Maximum number of HW directories per subject
pub const MAX_HW_DIRS: usize = 100;

/// A format specifier for the HW directory.
/// This format is used to search for existing HW directories and to create new HW directories.
///
/// Named arguments:
/// * `num` -- The HW number.
#[macro_export]
macro_rules! HW_DIR_FORMAT {
    () => {"HW{num}"};
}

/// A format specifier for the questions file.
/// This format is used to create the questions file when creating a new HW directory.
/// (The question file is moved from the most recent file in the downloads directory,
/// see [`DOWNLOADS_DIR`]. If it is `None`, no question file is created, and this value is ignored).
///
/// Named arguments:
/// * `original` -- The original name of the file in the downloads directory.
/// * `num` -- The HW number.
#[macro_export]
macro_rules! QUESTIONS_FILE_FORMAT {
    () => {"{original}_{num}"};
}

/// A format specifier for the LyX file.
/// This format is used to create a new LyX file when creating a new HW directory.
/// (The contents of the newly created LyX file will be copied from [`LYX_TEMPLATE_FILE`],
/// unless it is `None`, in which case no new LyX file is created, and this value is ignored).
///
/// Named arguments:
/// * `num` -- The HW number.
#[macro_export]
macro_rules! LYX_FILE_FORMAT {
    () => {"HW{num}"};
}

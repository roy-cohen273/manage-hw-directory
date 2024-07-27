mod config_values;

pub use config_values::{
    DOWNLOADS_DIR,
    SUBJECTS_DIR,
    LYX_TEMPLATE_FILE,
    MAX_HW_DIRS,
};

#[macro_export]
macro_rules! dummy_capture {
    ($($key:ident),*) => {
        concat!($(concat!("{", stringify!($key), "}")),*)
    }
}

#[macro_export]
macro_rules! cfg_func {
    ($($macro_name:ident -> $name:ident($($arg:ident:$T:ty),*);)*) => {
        $(
            pub fn $name($($arg: $T),*) -> String {
                let dummy_len = format!(
                    $crate::dummy_capture!($($arg),*),
                    $($arg=$arg),*
                )
                    .len();

                let mut s = format!(
                    concat!(
                        $crate::$macro_name!(),
                        $crate::dummy_capture!($($arg),*)
                    ),
                    $($arg=$arg),*
                );
                s.truncate(s.len() - dummy_len);
                s.shrink_to_fit();
                s
            }
        )*
    };
}

cfg_func! {
    HW_DIR_FORMAT -> get_hw_dir(num: usize);
    QUESTIONS_FILE_FORMAT -> get_questions_filname(num: usize, original: &str);
    LYX_FILE_FORMAT -> get_lyx_filename(num: usize);
}

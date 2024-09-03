use super::Interface;
use crate::files;
use crate::settings::Settings;
use cursive::{
    align::HAlign,
    view::Scrollable,
    views::{Dialog, SelectView, TextView},
    Cursive,
};

pub struct TuiInterface;

impl Interface for TuiInterface {
    fn main(settings: &Settings) -> anyhow::Result<()> {
        let subjects = files::get_subjects(settings)?.filter_map(|path| {
            let filename = path.file_name()?.to_str()?.to_owned();
            Some((path, filename))
        });

        let mut siv = cursive::default();

        let settings = settings.clone();
        let select = SelectView::new()
            .with_all(subjects.map(|(subject_path, subject_name)| {
                (subject_name.clone(), (subject_path, subject_name))
            }))
            .h_align(HAlign::Center)
            .autojump()
            .on_submit(move |siv, (subject_path, subject_name)| {
                let subject_path_open = subject_path.clone();
                let subject_path_new = subject_path.clone();
                let settings_open = settings.clone();
                let settings_new = settings.clone();

                siv.add_layer(
                    Dialog::around(TextView::new(subject_name))
                        .button("Cancel", move |siv| {
                            siv.pop_layer();
                        })
                        .button("Open", move |siv| {
                            if let Err(err) =
                                files::open_last_hw_dir(&settings_open, &subject_path_open)
                            {
                                error(siv, err);
                            }
                            siv.pop_layer();
                        })
                        .button("New", move |siv| {
                            if let Err(err) =
                                files::create_new_hw_dir(&settings_new, &subject_path_new)
                            {
                                error(siv, err);
                            }
                            siv.pop_layer();
                        }),
                )
            });

        siv.add_layer(Dialog::around(select.scrollable()).title("Pick a Subject"));
        siv.run();

        Ok(())
    }
}

fn error(siv: &mut Cursive, err: anyhow::Error) {
    // clear the screen. pop all layers.
    while siv.pop_layer().is_some() {}

    // error popup
    siv.add_layer(
        Dialog::text(err.to_string())
            .title("Error")
            .button("Quit", |siv| siv.quit()),
    );
}

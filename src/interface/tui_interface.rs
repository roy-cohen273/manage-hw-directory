use super::Interface;
use crate::files;
use crate::settings::Settings;
use cursive::{
    align::HAlign,
    event::{Callback, Event, EventResult},
    view::Scrollable,
    views::{Dialog, LinearLayout, OnEventView, PaddedView, SelectView, TextView},
    Cursive, With,
};

pub struct TuiInterface;

impl Interface for TuiInterface {
    fn main(settings: &Settings) -> anyhow::Result<()> {
        let subjects = files::get_subjects(settings)?.filter_map(|path| {
            let filename = path.file_name()?.to_str()?.to_owned();
            Some((path, filename))
        });

        let mut siv = cursive::default();

        let settings_select = settings.clone();
        let select = SelectView::new()
            .with_all(subjects.map(|(subject_path, subject_name)| {
                (subject_name.clone(), (subject_path, subject_name))
            }))
            .h_align(HAlign::Center)
            .autojump()
            .on_submit(move |siv, (subject_path, subject_name)| {
                let subject_path_open = subject_path.clone();
                let subject_path_new = subject_path.clone();
                let settings_open = settings_select.clone();
                let settings_new = settings_select.clone();

                siv.add_layer(
                    Dialog::text("Pick an action:")
                        .title(subject_name)
                        .button("Cancel", move |siv| {
                            siv.pop_layer();
                        })
                        .button("Open", move |siv| {
                            if let Err(err) =
                                files::open_last_hw_dir(&settings_open, &subject_path_open)
                            {
                                error(siv, &err);
                            } else {
                                siv.pop_layer();
                            }
                        })
                        .button("New", move |siv| {
                            if let Err(err) =
                                files::create_new_hw_dir(&settings_new, &subject_path_new)
                            {
                                error(siv, &err);
                            } else {
                                siv.pop_layer();
                            }
                        }),
                )
            });

        let settings_open_event = settings.clone();
        let settings_new_event = settings.clone();
        let select = OnEventView::new(select)
            .on_pre_event_inner(Event::CtrlChar('o'), move |select, _| {
                let selection = select.selection();
                let Some((subject_path, _subject_name)) = selection.as_deref() else {
                    return Some(EventResult::Consumed(None));
                };

                if let Err(err) = files::open_last_hw_dir(&settings_open_event, subject_path) {
                    return Some(EventResult::Consumed(Some(Callback::from_fn(move |siv| {
                        error(siv, &err);
                    }))));
                }

                Some(EventResult::Consumed(None))
            })
            .on_pre_event_inner(Event::CtrlChar('n'), move |select, _| {
                let selection = select.selection();
                let Some((subject_path, _subject_name)) = selection.as_deref() else {
                    return Some(EventResult::Consumed(None));
                };

                if let Err(err) = files::create_new_hw_dir(&settings_new_event, subject_path) {
                    return Some(EventResult::Consumed(Some(Callback::from_fn(move |siv| {
                        error(siv, &err);
                    }))));
                }

                Some(EventResult::Consumed(None))
            });

        siv.add_layer(
            LinearLayout::vertical()
                .child(
                    Dialog::around(select.scrollable())
                        .title("Pick a Subject")
                )
                .child(
                    TextView::new(
                        concat!(
                            "Press any letter to jump to the next subject beginning with that letter.\n",
                            "Press <Enter> to select a subject.\n",
                            "Press <Ctrl+O> to open the last HW directory.\n",
                            "Press <Ctrl+N> to create a new HW directory.\n",
                            "Press <Ctrl+C> to exit.",
                        )
                    )
                        .wrap_with(|text| PaddedView::lrtb(1, 1, 1, 1, text))
                )
        );
        siv.run();

        Ok(())
    }
}

fn error(siv: &mut Cursive, err: &anyhow::Error) {
    // clear the screen. pop all layers.
    while siv.pop_layer().is_some() {}

    // error popup
    siv.add_layer(
        Dialog::text(err.to_string())
            .title("Error")
            .button("Quit", |siv| siv.quit()),
    );
}

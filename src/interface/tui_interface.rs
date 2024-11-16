use super::Interface;
use crate::settings::Settings;
use crate::subject::Subject;
use cursive::{
    align::HAlign,
    event::{Callback, Event, EventResult},
    utils::markup::StyledString,
    view::{Nameable, Scrollable},
    views::{
        Dialog, LinearLayout, NamedView, OnEventView, PaddedView, SelectView, TextView, ViewRef,
    },
    Cursive, With,
};
use std::ops::DerefMut;

pub struct TuiInterface;

impl Interface for TuiInterface {
    fn main(settings: &Settings) -> anyhow::Result<()> {
        let subjects: Box<[Subject]> = Subject::get_all_subjects(settings)?;
        let subject_labels = subjects
            .iter()
            .map(|subject| settings.interface_settings().subject_label(subject))
            .collect::<Result<Box<[_]>, _>>()?;

        let mut siv = cursive::default();

        let select = SelectView::new()
            .with_all(
                subject_labels
                    .into_vec()
                    .into_iter()
                    .map(Into::<StyledString>::into)
                    .zip(subjects.into_vec()),
            )
            .h_align(HAlign::Center)
            .autojump()
            .on_submit({
                let settings = settings.clone();
                move |siv, subject: &Subject| {
                    siv.add_layer(
                        Dialog::text("Pick an action:")
                            .title(subject.name())
                            .button("Cancel", move |siv| {
                                siv.pop_layer();
                            })
                            .button("Open", move |siv| {
                                let mut select: ViewRef<SelectView<Subject>> =
                                    siv.find_name("select").unwrap();
                                let (_, subject) = selected(select.deref_mut());
                                if let Err(err) = subject.open_last_hw() {
                                    error(siv, &err);
                                } else {
                                    siv.pop_layer();
                                }
                            })
                            .button("New", {
                                let settings = settings.clone();
                                move |siv| {
                                    let mut select: ViewRef<SelectView<Subject>> =
                                        siv.find_name("select").unwrap();
                                    let (label, subject) = selected(select.deref_mut());

                                    match subject.create_new_hw_dir().and_then(|()| {
                                        settings
                                            .interface_settings()
                                            .subject_label(subject)
                                            .map_err(Into::into)
                                    }) {
                                        Ok(new_label) => {
                                            *label = new_label.into();
                                            siv.pop_layer();
                                        }
                                        Err(err) => {
                                            error(siv, &err);
                                        }
                                    }
                                }
                            }),
                    )
                }
            })
            .with_name("select");

        let select = OnEventView::new(select)
            .on_pre_event_inner(
                Event::CtrlChar('o'),
                move |select: &mut NamedView<SelectView<Subject>>, &_| {
                    let mut select = select.get_mut();
                    let Some((_, subject)) = try_selected(select.deref_mut()) else {
                        return Some(EventResult::Consumed(None));
                    };

                    if let Err(err) = subject.open_last_hw() {
                        return Some(EventResult::Consumed(Some(Callback::from_fn(move |siv| {
                            error(siv, &err);
                        }))));
                    }

                    Some(EventResult::Consumed(None))
                },
            )
            .on_pre_event_inner(Event::CtrlChar('n'), {
                let settings = settings.clone();
                move |select, _| {
                    let mut select = select.get_mut();
                    let Some((label, subject)) = try_selected(select.deref_mut()) else {
                        return Some(EventResult::Consumed(None));
                    };

                    match subject.create_new_hw_dir().and_then(|()| {
                        settings
                            .interface_settings()
                            .subject_label(subject)
                            .map_err(Into::into)
                    }) {
                        Ok(new_label) => *label = new_label.into(),
                        Err(err) => {
                            return Some(EventResult::Consumed(Some(Callback::from_fn(
                                move |siv| {
                                    error(siv, &err);
                                },
                            ))))
                        }
                    }

                    Some(EventResult::Consumed(None))
                }
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

fn try_selected(select: &mut SelectView<Subject>) -> Option<(&mut StyledString, &mut Subject)> {
    let i = select.selected_id()?;
    select.get_item_mut(i)
}

fn selected(select: &mut SelectView<Subject>) -> (&mut StyledString, &mut Subject) {
    try_selected(select).expect("some item should be selected")
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

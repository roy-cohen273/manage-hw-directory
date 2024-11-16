use super::Interface;
use crate::settings::Settings;
use crate::subject::Subject;
use std::io::{self, Write};

pub struct CmdInterface;

impl Interface for CmdInterface {
    fn main(settings: &Settings) -> anyhow::Result<()> {
        let mut subjects = Subject::get_all_subjects(settings)?;

        println!("List of available subjects:");
        for (i, subject) in subjects.iter().enumerate() {
            println!(
                "\t{i}. {}",
                settings.interface_settings().subject_label(subject)?,
            );
        }

        print!("Choose a subject: ");
        io::stdout().flush()?;
        let (open, subject) = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input = input.trim().to_owned();

            let open = input.starts_with("-o ");
            if open {
                input.replace_range(0..3, "");
            }

            if let Ok(i) = input.parse::<usize>() {
                if i < subjects.len() {
                    break (open, &mut subjects[i]);
                }
            }

            if let Some(subject) = subjects.iter_mut().find(|subject| subject.name() == input) {
                break (open, subject);
            }

            println!("That was not one of the options...");
            print!("Try again: ");
            io::stdout().flush()?;
        };

        if open {
            subject.open_last_hw()?;
        } else {
            subject.create_new_hw_dir()?;
        }

        Ok(())
    }
}

use super::Interface;
use crate::config::Config;
use crate::files;
use std::io;
use std::io::Write;

pub struct CmdInterface;

impl Interface for CmdInterface {
    fn main(config: &Config) -> anyhow::Result<()> {
        let subject_paths: Box<[_]> = files::get_subjects(config)?.collect();
        let subjects: Box<[_]> = subject_paths
            .iter()
            .filter_map(|path| Some((path, path.file_name()?.to_str()?)))
            .collect();

        println!("List of available subjects:");
        for (i, (_subject_path, subject_filename)) in subjects.iter().enumerate() {
            println!("\t{i}. {subject_filename}");
        }

        print!("Choose a subject: ");
        io::stdout().flush()?;
        let (open, subject_dir) = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input = input.trim().to_owned();

            let open = input.starts_with("-o ");
            if open {
                input.replace_range(0..3, "");
            }

            if let Ok(i) = input.parse::<usize>() {
                if i < subjects.len() {
                    break (open, subjects[i].0);
                }
            }

            if let Some((subject_path, _subject_name)) = subjects
                .iter()
                .find(|(_subject_path, subject_name)| *subject_name == input)
            {
                break (open, *subject_path);
            }

            println!("That was not one of the options...");
            print!("Try again: ");
            io::stdout().flush()?;
        };

        if open {
            files::open_last_hw_dir(config, subject_dir)?;
        } else {
            files::create_new_hw_dir(config, subject_dir)?;
        }

        Ok(())
    }
}

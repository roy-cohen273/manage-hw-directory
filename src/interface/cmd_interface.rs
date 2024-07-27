use super::Interface;
use crate::config::Config;
use crate::files;
use std::io;
use std::io::Write;

pub struct CmdInterface;

impl Interface for CmdInterface {
    type Error = io::Error;

    fn main(config: &Config) -> Result<(), Self::Error> {
        let subject_paths: Vec<_> = files::get_subjects(config)?.collect();
        let subjects: Vec<_> = subject_paths
            .iter()
            .filter_map(|path| path.file_name().and_then(|s| s.to_str()))
            .collect();

        println!("List of available subjects:");
        for subject in &subjects {
            println!("\t{subject}");
        }

        print!("Choose a subject: ");
        io::stdout().flush()?;
        let subject_dir = loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            if let Some(path) = subject_paths.iter().find(|path| {
                path.file_name()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s == input.trim())
            }) {
                break path;
            }
            println!("That was not one of the options...");
            print!("Try again: ");
            io::stdout().flush()?;
        };

        files::do_the_thing(config, subject_dir)?;

        Ok(())
    }
}

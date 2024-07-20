use std::io;

mod files;


fn main() -> io::Result<()> {
    let subject_paths: Vec<_> = files::get_subjects()?.collect();
    let subjects = subject_paths.iter()
        .filter_map(|path| path.file_name().and_then(|s| s.to_str()));
    println!("List of available subjects:");
    for subject in subjects {
        println!("\t{subject}");
    }

    let subject = "sub2";
    files::do_the_thing(subject)?;

    Ok(())
}

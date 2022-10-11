use walkdir::WalkDir;
use supports_color::Stream;
use colored::*;

fn main() {
    for entry in WalkDir::new("/sys/kernel").into_iter() {
        match entry {
            Ok(entry) => {
                if !entry.file_type().is_dir() {
                    println!("{}", entry.path().display());
                }
            },
            Err(e) => {
                colored::control::set_override(supports_color::on(Stream::Stderr).is_some());
                eprintln!("{}", e.to_string().red());
            }
        }
    }
}

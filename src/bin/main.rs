use mpd;
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("assets/data/resume.md");
    let markdown = fs::read_to_string(file_path).expect("Failed to read test file");
    mpd::parse(markdown);
}

use mpd;
use std::{fs, path::Path};

fn main() {
    let file_path = Path::new("src/lib/test_data/nt.md");
    let markdown = fs::read_to_string(file_path).expect("Failed to read test file");
    println!("{}", markdown);
    mpd::parse(markdown);
}

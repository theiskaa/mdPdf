use clap::{Arg, Command};
use std::{fs, env};

fn main() {
    let matches = Command::new("Markdown to PDF Converter")
        .version("1.0")
        .about("Converts Markdown files or strings to PDF")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("FILE_PATH")
                .help("Path to the markdown file")
                .required(false),
        )
        .arg(
            Arg::new("string")
                .short('s')
                .long("string")
                .value_name("MARKDOWN_STRING")
                .help("Markdown content as a string")
                .required(false),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT_PATH")
                .help("Path to the output PDF file")
                .required(false),
        )
        .get_matches();

    let markdown = if let Some(file_path) = matches.get_one::<String>("path") {
        fs::read_to_string(file_path).expect("[X] Failed to read markdown file")
    } else if let Some(markdown_string) = matches.get_one::<String>("string") {
        markdown_string.to_string()
    } else {
        let current_dir = env::current_dir().unwrap();
        let help = fs::read_to_string(current_dir.join("src/bin/help.txt")).unwrap();
        println!("{}", help);
        return;
    };

    let binding = "output.pdf".to_string();
    let output_path = matches.get_one::<String>("output").unwrap_or(&binding);

    mdp::parse(markdown, output_path);

    println!("PDF saved to: {}", output_path);
}

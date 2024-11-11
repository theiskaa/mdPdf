use clap::{Arg, Command};
use std::fs;

// TODO: move this help message into another file (txt)
const HELP: &str = r#"

                _      _                  ___           _  __
 _ __  __ _ _ _| |____| |_____ __ ___ _  |_  ) _ __  __| |/ _|
| '  \/ _` | '_| / / _` / _ \ V  V / ' \  / / | '_ \/ _` |  _|
|_|_|_\__,_|_| |_\_\__,_\___/\_/\_/|_||_|/___\| .__/\__,_|_|
                                              |_|


Usage: markdown2pdf [OPTIONS]
The 'markdown2pdf' command is a tool for converting Markdown content into a PDF document.

Options:
  -p, --path        Specify the path to the Markdown file to convert.
  -s, --string      Provide Markdown content directly as a string.
  -o, --output      Specify the output file path for the generated PDF.

Examples:
  markdown2pdf -p "docs/resume.md" -o "resume.pdf"
     Convert the 'resume.md' file in the 'docs' folder to 'resume.pdf'.

  markdown2pdf -s "**bold text** *italic text*." -o "output.pdf"
     Convert the provided Markdown string to 'output.pdf'.

  markdown2pdf -p "file.md"
     Convert 'file.md' to a PDF, saving it as 'output.pdf'.

Notes:
- If both `-p` and `-s` options are provided, the `--path` option will take precedence.
- If no output file is specified with `-o`, the default output file will be 'output.pdf'.
- Source code can be viewed at: https://github.com/theiskaa/markdown2pdf
"#;

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
        println!("{}", HELP);
        return;
    };

    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let binding = current_dir.join("output.pdf");
    let output_path = matches
        .get_one::<String>("output")
        .map(|p| current_dir.join(p))
        .unwrap_or(binding);
    let output_path = output_path.to_str().expect("Invalid output path");

    let result = markdown2pdf::parse(markdown, output_path);
    match result {
        Ok(_) => println!("[OK] Saved PDF to {}", output_path),
        Err(e) => println!("[ERROR] Failed to transpile markdown:\n> {}", e),
    }
}

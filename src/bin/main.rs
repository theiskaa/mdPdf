use clap::{Arg, Command};
use std::fs;

// TODO: move this help message into another file (txt)
const HELP: &str = r#"
          _ ___
 _ __  __| | _ \
| '  \/ _` |  _/
|_|_|_\__,_|_|


Usage: mdp [OPTIONS]
The 'mdp' command is a tool for converting Markdown content into a PDF document.

Options:
  -p, --path        Specify the path to the Markdown file to convert.
  -s, --string      Provide Markdown content directly as a string.
  -o, --output      Specify the output file path for the generated PDF.

Examples:
  mdp -p "docs/resume.md" -o "resume.pdf"
     Convert the 'resume.md' file in the 'docs' folder to 'resume.pdf'.

  mdp -s "**bold text** *italic text*." -o "output.pdf"
     Convert the provided Markdown string to 'output.pdf'.

  mdp -p "file.md"
     Convert 'file.md' to a PDF, saving it as 'output.pdf'.

Notes:
- If both `-p` and `-s` options are provided, the `--path` option will take precedence.
- If no output file is specified with `-o`, the default output file will be 'output.pdf'.
- Source code can be viewed at: https://github.com/theiskaa/mdp
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

    let binding = "output.pdf".to_string();
    let output_path = matches.get_one::<String>("output").unwrap_or(&binding);

    let _ = mdp::parse(markdown, output_path);

    println!("PDF saved to: {}", output_path);
}

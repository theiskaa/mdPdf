# mdPdf
mdPdf is a versatile command-line tool and library designed to convert Markdown content into pre-styled PDF documents. It supports reading Markdown from a file or directly from a string, offering flexibility for both users and developers.

This project includes both a binary and a library:
- **Binary (cli)**: A command-line interface that uses the core library to provide an easy way to convert Markdown to PDF.
- **Library (lib)**: Can be integrated into your Rust projects for parsing Markdown or generating PDF documents programmatically.

> **Note:** This project is currently under active development, with ongoing improvements and new features being added.

## Installation
Currently, there isn't a simplified official method to install the command-line tool across platforms. To use it, you can clone the repository and build the project yourself using Cargo:

```bash
git clone https://github.com/theiskaa/mdPdf.git
cd mdPdf
cargo build --release
```

## Usage

To use the `mdPdf` tool, you can either specify a Markdown file path, provide Markdown content directly, or set the output PDF path.

### Options

- `-p`, `--path`: Specify the path to the Markdown file to convert.
- `-s`, `--string`: Provide Markdown content directly as a string.
- `-o`, `--output`: Specify the output file path for the generated PDF.

### Examples

1. Convert a Markdown file to a PDF:

   ```bash
   mdp -p "docs/resume.md" -o "resume.pdf"
   ```

   This will convert the `resume.md` file located in the `docs` folder to a PDF file named `resume.pdf`.

2. Convert Markdown content provided as a string:

   ```bash
   mdp -s "# My Markdown Title\n\nThis is content." -o "output.pdf"
   ```

   This will convert the provided Markdown string to a PDF file named `output.pdf`.

3. Convert a Markdown file to a PDF with default output:

   ```bash
   mdp -p "file.md"
   ```

   This will convert the `file.md` to a PDF and save it as `output.pdf`.

### Notes

- If both `-p` and `-s` options are provided, the `--path` option will take precedence.
- If no output file is specified using the `-o` option, the default output file will be named `output.pdf`.

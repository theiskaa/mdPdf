# markdown2pdf
markdown2pdf is a versatile command-line tool and library designed to convert Markdown content into pre-styled PDF documents. It supports reading Markdown from a file or directly from a string, offering flexibility for both users and developers.

This project includes both a binary and a library:
- **Binary (cli)**: A command-line interface that uses the core library to provide an easy way to convert Markdown to PDF.
- **Library (lib)**: Can be integrated into your Rust projects for parsing Markdown or generating PDF documents programmatically.

> **Note:** This project is currently under active development, with ongoing improvements and new features being added.

## Installation
You can install the `markdown2pdf` binary globally using cargo by running:
```bash
cargo install markdown2pdf
```

If you want to install the latest git version:
```bash
cargo install --git https://github.com/theiskaa/markdown2pdf
```

Alternatively, you can build from source by cloning the repository and using `Makefile`:
```bash
git clone https://github.com/theiskaa/markdown2pdf.git
cd markdown2pdf
make build-in-local
```

## Usage
To use the `markdown2pdf` tool, you can either specify a Markdown file path, provide Markdown content directly, or set the output PDF path.

### Options

- `-p`, `--path`: Specify the path to the Markdown file to convert.
- `-s`, `--string`: Provide Markdown content directly as a string.
- `-o`, `--output`: Specify the output file path for the generated PDF.

### Examples

1. Convert a Markdown file to a PDF:
   ```bash
   markdown2pdf -p "docs/resume.md" -o "resume.pdf"
   ```

   This will convert the `resume.md` file located in the `docs` folder to a PDF file named `resume.pdf`.

2. Convert Markdown content provided as a string:
   ```bash
   markdown2pdf -s "# My Markdown Title\n\nThis is content." -o "output.pdf"
   ```

   This will convert the provided Markdown string to a PDF file named `output.pdf`.

3. Convert a Markdown file to a PDF with default output:
   ```bash
   markdown2pdf -p "file.md"
   ```

   This will convert the `file.md` to a PDF and save it as `output.pdf`.

## Configuration
The `markdown2pdf` tool supports customization through a TOML configuration file. You can configure various styling options for the generated PDFs by creating a `markdown2pdfrc.toml` file in your home directory. To get started with configuration:

1. Create the config file:
   ```bash
   touch ~/markdown2pdfrc.toml
   ```

2. Copy the example configuration:
   - View the example config at [markdown2pdfrc.example.toml](markdown2pdfrc.example.toml)
   - Copy the contents to your `~/markdown2pdfrc.toml` file
   - Modify the values according to your preferences

The configuration file allows you to customize the appearance of your generated PDFs by defining styling options for different Markdown elements.

### Notes
- If both `-p` and `-s` options are provided, the `--path` option will take precedence.
- If no output file is specified using the `-o` option, the default output file will be named `output.pdf`.

## Contributing
For information regarding contributions, please refer to [CONTRIBUTING.md](CONTRIBUTING.md) file.

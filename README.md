# markdown2pdf
markdown2pdf is a versatile command-line tool and library designed to convert Markdown content into pre-styled PDF documents. It supports reading Markdown from a file or directly from a string, offering flexibility for both users and developers.

This project includes both a binary and a library:
- **Binary (cli)**: A command-line interface that uses the core library to provide an easy way to convert Markdown to PDF.
- **Library (lib)**: Can be integrated into your Rust projects for parsing Markdown or generating PDF documents programmatically.

> **Note:** This project is currently under active development, with ongoing improvements and new features being added.

### Install

You can install the `markdown2pdf` binary globally using cargo by running:
```bash
cargo install markdown2pdf
```

If you want to install the latest git version:
```bash
cargo install --git https://github.com/theiskaa/markdown2pdf
```

### Install as library

Run the following Cargo command in your project directory:
```bash
cargo add markdown2pdf
```

Or add the following line to your Cargo.toml:
```toml
markdown2pdf = "0.1.2"
```

## Usage
To use the `markdown2pdf` tool, you can either specify a Markdown file path, provide Markdown content directly, or set the output PDF path.
### Options
- `-p`, `--path`: Specify the path to the Markdown file to convert.
- `-s`, `--string`: Provide Markdown content directly as a string.
- `-u`, `--url`: Specify a URL to fetch Markdown content from.
- `-o`, `--output`: Specify the output file path for the generated PDF.

### Examples
1. Convert a Markdown file to a PDF:
   ```bash
   markdown2pdf -p "docs/resume.md" -o "resume.pdf"
   ```

   Convert the 'resume.md' file in the 'docs' folder to 'resume.pdf'.

2. Convert Markdown content provided as a string:
   ```bash
   markdown2pdf -s "**bold text** *italic text*." -o "output.pdf"
   ```

   Convert the provided Markdown string to 'output.pdf'.

3. Convert Markdown from a URL:
   ```bash
   markdown2pdf -u "https://raw.githubusercontent.com/user/repo/main/README.md" -o "readme.pdf"
   ```

   Convert the Markdown content from the URL to 'readme.pdf'.

### Notes
- If multiple input options (-p, -s, -u) are provided, only one will be used in this order: path > url > string
- If no output file is specified with `-o`, the default output file will be 'output.pdf'.

## Using as Library
The library at its core employs a lexical analyzer that tokenizes the input Markdown text into semantic elements like headings, emphasis, code blocks, and lists. The lexer handles nested structures and maintains proper context during parsing. After lexical analysis, the tokens are processed through a styling engine that applies visual formatting based on configuration rules. The styling engine supports customization of fonts, colors, spacing, and other typographic properties through a TOML configuration file.

The library exposes a high-level `parse()` function that orchestrates the entire conversion process. This function accepts raw Markdown text and an output path, handling all intermediate processing steps internally. Under the hood, it leverages the lexer to build an abstract syntax tree, applies styling rules from configuration, and renders the final PDF output. For basic usage, simply pass your Markdown content as a string to `parse()`.

For more advanced usage, you can work directly with the lexer and PDF generation components. First, create a lexer instance to parse your Markdown content into tokens
```rust
let mut lexer = Lexer::new(markdown);
let tokens = lexer.parse().unwrap(); // handle errors
```

Next, you'll need to create a PDF renderer to transform the tokens into a formatted document. Before initializing the renderer, you'll need to define styling rules through a `StyleMatch` instance. The `StyleMatch` determines how different Markdown elements will be rendered in the final PDF. You can use the default styling that comes with the library, but for more control over the appearance, you can create your own custom `StyleMatch` implementation or load styling rules from a configuration file as described in the Configuration section down below. This gives you fine-grained control over fonts, colors, spacing and other visual properties of your PDF output.
```rust
let style = config::load_config();
let pdf = Pdf::new(tokens, style);
let document = pdf.render_into_document();
```

Finally, the `Document` object can be rendered to a PDF file using the `Pdf::render()` function. This function handles the actual PDF generation, applying all the styling rules and formatting defined earlier. It takes the output path as a parameter and returns a `Result` indicating success or any errors that occurred during rendering:

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

## Contributing
For information regarding contributions, please refer to [CONTRIBUTING.md](CONTRIBUTING.md) file.

use crate::{fonts::MdPdfFont, Token};

#[allow(dead_code)]
pub struct Pdf {
    input: Vec<Token>,
}

impl Pdf {
    pub fn new(input: Vec<Token>) -> Self {
        Self { input }
    }

    // TODO: implement the end to end token to pdf element transformation
    pub fn create_document(self) -> genpdf::Document {
        let font_family = genpdf::fonts::from_files(
            "assets/fonts",
            MdPdfFont::ITCAvantGardeGothicStdMedium.name(),
            None,
        )
        .expect("Failed to load font family");

        let mut doc = genpdf::Document::new(font_family);
        doc.set_title("Demo document");

        // Customize the pages
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);

        // Add one or more elements
        doc.push(genpdf::elements::Paragraph::new("This is a demo document."));
        return doc;
    }

    pub fn render(document: genpdf::Document, file: &str) {
        match document.render_to_file(file) {
            Ok(_) => {
                println!("Successfully saved your pdf to {}", file);
            }
            Err(err) => {
                println!("Failed to save file to {}: {}", file, err);
            }
        }
    }
}

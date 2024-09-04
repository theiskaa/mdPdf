#[derive(Debug, PartialEq)]
pub enum MdPdfFont {
    ITCAvantGardeGothicStdMedium,
}

impl MdPdfFont {
    pub fn name(&self) -> &'static str {
        match self {
            MdPdfFont::ITCAvantGardeGothicStdMedium => "ITC-Avant-Garde-Gothic-Std-Medium",
        }
    }
}

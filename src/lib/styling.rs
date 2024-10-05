use genpdf::{
    error::Error,
    fonts::{FontData, FontFamily},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MdPdfFont {
    ITCAvantGardeGothicStdMedium,
}

impl MdPdfFont {
    pub fn name(&self) -> &'static str {
        match self {
            MdPdfFont::ITCAvantGardeGothicStdMedium => "ITC-Avant-Garde-Gothic-Std-Medium",
        }
    }
    pub fn find_match(family: Option<&str>) -> MdPdfFont {
        match family.unwrap_or("ITC-Avant-Garde-Gothic-Std-Medium") {
            "ITC-Avant-Garde-Gothic-Std-Medium" => MdPdfFont::ITCAvantGardeGothicStdMedium,
            _ => MdPdfFont::ITCAvantGardeGothicStdMedium,
        }
    }

    pub fn load_font_family(family: Option<&str>) -> Result<FontFamily<FontData>, Error> {
        let found_match = MdPdfFont::find_match(family);
        genpdf::fonts::from_files("assets/fonts", found_match.name(), None)
    }
}

#[derive(Clone, Copy)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Clone, Copy)]
pub struct Margins {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

#[derive(Clone, Copy)]
pub struct BasicTextStyle {
    pub size: u8,
    pub text_color: Option<(u8, u8, u8)>,
    pub after_spacing: f32,
    pub alignment: Option<TextAlignment>,
    pub font_family: Option<&'static str>,
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub background_color: Option<(u8, u8, u8)>,
}

impl BasicTextStyle {
    pub fn new(
        size: u8,
        text_color: Option<(u8, u8, u8)>,
        after_spacing: Option<f32>,
        alignment: Option<TextAlignment>,
        font_family: Option<&'static str>,
        bold: bool,
        italic: bool,
        underline: bool,
        strikethrough: bool,
        background_color: Option<(u8, u8, u8)>,
    ) -> Self {
        Self {
            size,
            text_color,
            after_spacing: after_spacing.unwrap_or(0.0),
            alignment,
            font_family,
            bold,
            italic,
            underline,
            strikethrough,
            background_color,
        }
    }
}

pub struct StyleMatch {
    pub margins: Margins,
    pub heading_1: BasicTextStyle,
    pub heading_2: BasicTextStyle,
    pub heading_3: BasicTextStyle,
    pub emphasis: BasicTextStyle,
    pub strong_emphasis: BasicTextStyle,
    pub code: BasicTextStyle,
    pub block_quote: BasicTextStyle,
    pub list_item: BasicTextStyle,
    pub link: BasicTextStyle,
    pub image: BasicTextStyle,
    pub text: BasicTextStyle,
    pub horizontal_rule: BasicTextStyle,
}

impl StyleMatch {
    pub fn default() -> Self {
        Self {
            margins: Margins {
                top: 10.0,
                right: 10.0,
                bottom: 10.0,
                left: 10.0,
            },
            heading_1: BasicTextStyle::new(
                16,
                Some((0, 0, 0)),
                Some(0.5),
                Some(TextAlignment::Center),
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            heading_2: BasicTextStyle::new(
                14,
                Some((0, 0, 0)),
                Some(0.5),
                Some(TextAlignment::Left),
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            heading_3: BasicTextStyle::new(
                12,
                Some((0, 0, 0)),
                Some(0.5),
                Some(TextAlignment::Left),
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            emphasis: BasicTextStyle::new(
                10,
                Some((0, 0, 0)),
                None,
                None,
                None,
                false,
                true,
                false,
                false,
                None,
            ),
            strong_emphasis: BasicTextStyle::new(
                10,
                Some((0, 0, 0)),
                None,
                None,
                None,
                true,
                false,
                false,
                false,
                None,
            ),
            code: BasicTextStyle::new(
                10,
                Some((128, 128, 128)),
                None,
                None,
                Some("ITC-Avant-Garde-Gothic-Std-Medium"),
                false,
                false,
                false,
                false,
                Some((230, 230, 230)),
            ),
            block_quote: BasicTextStyle::new(
                10,
                Some((128, 128, 128)),
                None,
                None,
                None,
                false,
                true,
                false,
                false,
                Some((245, 245, 245)),
            ),
            list_item: BasicTextStyle::new(
                10,
                Some((0, 0, 0)),
                Some(0.5),
                None,
                None,
                false,
                false,
                false,
                false,
                None,
            ),
            link: BasicTextStyle::new(
                10,
                Some((128, 128, 128)),
                None,
                None,
                None,
                false,
                false,
                true,
                false,
                None,
            ),
            image: BasicTextStyle::new(
                10,
                Some((0, 0, 0)),
                None,
                Some(TextAlignment::Center),
                None,
                false,
                false,
                false,
                false,
                None,
            ),
            text: BasicTextStyle::new(
                10,
                Some((0, 0, 0)),
                None,
                None,
                None,
                false,
                false,
                false,
                false,
                None,
            ),
            horizontal_rule: BasicTextStyle::new(
                10,
                Some((0, 0, 0)),
                Some(0.5),
                None,
                None,
                false,
                false,
                false,
                false,
                None,
            ),
        }
    }
}

#![allow(dead_code)]

pub struct BasicTextStyle {
    size: f32,
    rgb_color: Option<(u8, u8, u8)>,
}

pub struct StyleMatch {
    heading_1: BasicTextStyle,
    heading_2: BasicTextStyle,
    heading_3: BasicTextStyle,
    emphasis: BasicTextStyle,
    strong_emphasis: BasicTextStyle,
    code: BasicTextStyle,
    // TODO: needs proper styling with background color
    block_quote: BasicTextStyle,

    // TODO: needs proper styling, a way to choose between dot or slash
    list_item: BasicTextStyle,

    // TODO: needs proper styling, a way to choose between dot or slash
    link: BasicTextStyle,
    image: BasicTextStyle,
    text: BasicTextStyle,
    newline: BasicTextStyle,
    horizontal_rule: BasicTextStyle,
}

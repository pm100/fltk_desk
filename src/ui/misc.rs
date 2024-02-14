use fltk::enums::{Color, Font};

pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub hl: Color,
    pub frame_color: Color,
    pub font: Font,
    pub font_size: i32,
    pub mono_font: Font,
    pub mono_font_size: i32,
}

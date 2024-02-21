use fltk::enums::{Color, Font};
#[derive(Copy, Clone, Debug)]
pub struct Theme {
    pub bg: Color,
    pub fg: Color,
    pub popbg: Color,
    pub hl: Color,
    pub frame_color: Color,
    pub font: Font,
    pub font_size: i32,
    pub mono_font: Font,
    pub mono_font_size: i32,
}
// pub struct Environment<TM: Send + Sync + Clone + 'static> {
//     pub theme: &'static Theme,
//     pub channel: fltk::app::Sender<TM>,
// }

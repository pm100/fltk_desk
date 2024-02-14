use fltk::{
    browser::HoldBrowser,
    draw::Rect,
    enums::{Color, Font, FrameType},
    group::Group,
    prelude::{WidgetBase, *},
    text::{TextBuffer, TextDisplay},
    widget::Widget,
};

use super::{control::Control, misc::Theme};

pub struct TextBoxx<Message>
where
    Message: Send + Sync + Clone + 'static,
{
    text: TextDisplay,
    theme: &'static Theme,
    buffer: TextBuffer,
    phantom: std::marker::PhantomData<Message>,
}

impl<Message> TextBoxx<Message>
where
    Message: Send + Sync + Clone + 'static,
{
    pub fn new(size: Rect, theme: &'static Theme) -> Self {
        let mut text_display = TextDisplay::new(size.x, size.y, size.w, size.h, "");

        text_display.set_color(theme.bg);
        text_display.set_text_color(theme.fg);
        text_display.set_text_font(theme.mono_font);
        text_display.set_text_size(theme.mono_font_size);

        text_display.set_frame(FrameType::FlatBox);
        let buffer = TextBuffer::default();
        text_display.set_buffer(buffer.clone());

        text_display.handle(|x, y| {
            println!("x: {:?}, y: {:?}", x, y);
            true
        });
        let s = Self {
            text: text_display,
            buffer: buffer,
            theme,
            phantom: std::marker::PhantomData,
        };

        s
    }
    pub fn clear(&mut self) {
        self.buffer.set_text("");
    }
    pub fn set_text(&mut self, text: &str) {
        self.buffer.set_text(text);
    }
    pub fn append(&mut self, text: &str) {
        self.buffer.append(text);
    }
}
#[derive(Clone)]
pub struct TextBox {
    text: HoldBrowser,
    theme: &'static Theme,
}

impl TextBox {
    pub fn new(size: Rect, theme: &'static Theme) -> Self {
        Group::set_current(None::<&Group>);
        let mut text = HoldBrowser::new(size.x, size.y, size.w, size.h, "");
        text.set_color(theme.bg);
        text.set_selection_color(theme.hl);

        text.set_text_size(theme.mono_font_size);
        text.set_frame(FrameType::FlatBox);
        let s = Self { text, theme };
        s
    }
    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn append(&mut self, text: &str) {
        let line = format!(
            "@C{}@F{}@S{}@.{}",
            self.theme.fg.bits(),
            self.theme.font.bits(),
            self.theme.font_size,
            text
        );
        self.text.add(&line);
    }
}
impl Control for TextBox {
    fn fl_widget(&self) -> Widget {
        let x = unsafe {
            Widget::from_widget_ptr(self.text.as_widget_ptr() as *mut fltk_sys::widget::Fl_Widget)
        };
        x
    }
}

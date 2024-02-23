use fltk::{
    browser::HoldBrowser,
    draw::Rect,
    enums::{Color, Font, FrameType},
    group::Group,
    prelude::{WidgetBase, *},
    text::{TextBuffer, TextDisplay},
    widget::Widget,
};

use super::{
    application::{ApplicationExt, ApplicationPtr},
    control::Control,
    misc::Theme,
};

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
pub struct TextBox<TM: Send + Sync + 'static> {
    text: HoldBrowser,
    //theme: &'static Theme,
    app: ApplicationPtr<TM>,
}

impl<TM: Send + Sync + 'static> TextBox<TM> {
    pub fn new(size: Rect, app: &ApplicationPtr<TM>) -> Self {
        Group::set_current(None::<&Group>);
        let mut text = HoldBrowser::new(size.x, size.y, size.w, size.h, None);
        let theme = app.get_theme();
        text.set_color(theme.bg);
        text.set_selection_color(theme.hl);

        text.set_text_size(theme.mono_font_size);
        text.set_frame(FrameType::DownBox);
        let s = Self {
            text,
            app: app.clone(),
        };
        s
    }
    pub fn clear(&mut self) {
        self.text.clear();
    }

    pub fn append(&mut self, text: &str) {
        let theme = &self.app.get_theme();
        let line = format!(
            "@C{}@F{}@S{}@.{}",
            theme.fg.bits(),
            theme.font.bits(),
            theme.font_size,
            text
        );
        self.text.add(&line);
    }
}
impl<TM> Control<TM> for TextBox<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    fn fl_widget(&self) -> Widget {
        let x = unsafe {
            Widget::from_widget_ptr(self.text.as_widget_ptr() as *mut fltk_sys::widget::Fl_Widget)
        };
        x
    }
}

use fltk::{
    draw::Rect,
    prelude::{GroupExt, WidgetExt},
};

use super::application::ApplicationPtr;
use super::{application::ApplicationExt, control::Control};

pub struct MainWin<TM: Send + Sync + Clone + 'static> {
    pub(crate) win: fltk::window::Window,
    app: ApplicationPtr<TM>,
    phantom: std::marker::PhantomData<TM>,
}

impl<TM: Send + Sync + Clone + 'static> MainWin<TM> {
    pub fn new(rect: Rect, app: &ApplicationPtr<TM>) -> Self {
        let mut win = fltk::window::Window::default().with_size(rect.w, rect.h);
        win.set_color(app.get_theme().bg);
        let s = Self {
            win,
            phantom: std::marker::PhantomData,
            app: app.clone(),
        };
        s
    }
    pub fn show(&mut self) {
        self.win.make_resizable(true);
        self.win.show();
    }
    pub fn end(&mut self) {
        self.win.end();
    }
    pub fn add(&mut self, widget: &impl Control<TM>) {
        self.win.add(&widget.fl_widget())
    }
}

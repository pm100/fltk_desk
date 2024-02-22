use fltk::{
    app,
    enums::{Color, FrameType},
    prelude::{MenuExt, WidgetBase, WidgetExt},
    widget::Widget,
};

use super::{
    application::{ApplicationExt, ApplicationPtr},
    control::Control,
};

pub struct Menu<TM: Send + Sync + Clone + 'static> {
    app: ApplicationPtr<TM>,
    menu: fltk::menu::SysMenuBar,
    phantom: std::marker::PhantomData<TM>,
}
impl<TM: Send + Sync + Clone + 'static> Menu<TM> {
    pub fn new(app: &ApplicationPtr<TM>) -> Self {
        let mut menu = fltk::menu::SysMenuBar::default().with_size(800, 34);
        let mut s = Self {
            app: app.clone(),
            menu,
            phantom: std::marker::PhantomData,
        };

        app::set_menu_linespacing(15);
        s.menu.set_color(app.get_theme().popbg);
        s.menu.set_text_color(app.get_theme().fg);
        s.menu.set_frame(FrameType::FlatBox);
        s.menu.set_selection_color(app.get_theme().hl);

        app::set_contrast_mode(app::ContrastMode::None);
        s
    }
    pub fn add(&mut self, label: &str, shortcut: fltk::enums::Shortcut, message: TM) {
        self.menu
            .add(label, shortcut, fltk::menu::MenuFlag::Normal, {
                let m = self.app.get_sender().clone();
                move |item| {
                    m.send(message.clone());
                }
            });
    }
}
impl<TM> Control<TM> for Menu<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    fn fl_widget(&self) -> Widget {
        let x = unsafe {
            Widget::from_widget_ptr(self.menu.as_widget_ptr() as *mut fltk_sys::widget::Fl_Widget)
        };
        x
    }
}

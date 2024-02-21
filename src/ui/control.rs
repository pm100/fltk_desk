use fltk::{prelude::WidgetExt, widget::Widget};


pub trait Control<TM: Send + Sync + Clone + 'static> {
    fn fl_widget(&self) -> Widget;
    // fn get_env(&self) -> &'static Environment<TM>;
}

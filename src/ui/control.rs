use fltk::{prelude::WidgetExt, widget::Widget};

pub trait Control {
    fn fl_widget(&self) -> Widget;
}

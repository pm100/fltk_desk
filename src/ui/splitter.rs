use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, rc::Rc};

use super::{control::Control, misc::Theme};
use fltk::{
    draw::Rect,
    group::{Group, Tile},
    prelude::{GroupExt, WidgetBase, WidgetExt},
    widget::Widget,
};
use fltk_sys;
#[derive(Clone)]
pub struct Splitter {
    tile: Tile,
    theme: &'static Theme,
    panes: Rc<RefCell<Vec<Box<dyn Control>>>>,
    vertical: bool,
}

impl Control for Splitter {
    fn fl_widget(&self) -> Widget {
        let x = unsafe {
            Widget::from_widget_ptr(self.tile.as_widget_ptr() as *mut fltk_sys::widget::Fl_Widget)
        };
        x
    }
}

impl Splitter {
    pub fn new(rect: Rect, theme: &'static Theme, vertical: bool) -> Splitter {
        Group::set_current(None::<&Group>);
        let s = Splitter {
            tile: Tile::new(rect.x, rect.y, rect.w, rect.h, None),
            theme,
            panes: Rc::default(),
            vertical,
        };

        s.tile.end();
        s
    }

    pub fn add<W>(&mut self, w: W)
    where
        W: Control + Clone + 'static,
    {
        self.panes.deref().borrow_mut().push(Box::new(w.clone()));
        if self.vertical {
            let childwidth = self.tile.w() / (self.panes.deref().borrow_mut().len()) as i32;
            for (i, p) in self.panes.deref().borrow_mut().iter_mut().enumerate() {
                p.fl_widget().resize(
                    self.tile.x() + (i as i32 * childwidth),
                    self.tile.y(),
                    childwidth,
                    self.tile.h(),
                );
            }
        } else {
            let childheight = self.tile.h() / (self.panes.deref().borrow().len()) as i32;
            for (i, p) in self.panes.deref().borrow_mut().iter_mut().enumerate() {
                p.fl_widget().resize(
                    self.tile.x(),
                    self.tile.y() + (i as i32 * childheight),
                    self.tile.w(),
                    childheight,
                );
            }
        }

        self.tile.add(&w.fl_widget());
    }
}

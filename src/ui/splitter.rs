use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, rc::Rc};

use super::{application::ApplicationPtr, control::Control, misc::Theme};
use fltk::{
    draw::Rect,
    enums::FrameType,
    frame::Frame,
    group::{Group, Tile},
    prelude::{GroupExt, WidgetBase, WidgetExt},
    widget::Widget,
};
use fltk_sys;
#[derive(Clone)]
pub struct Splitter<TM: Send + Sync + Clone + 'static> {
    tile: Tile,
    app: ApplicationPtr<TM>,
    panes: Rc<RefCell<Vec<Box<dyn Control<TM>>>>>,
    vertical: bool,
}

impl<TM> Control<TM> for Splitter<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    fn fl_widget(&self) -> Widget {
        let x = unsafe {
            Widget::from_widget_ptr(self.tile.as_widget_ptr() as *mut fltk_sys::widget::Fl_Widget)
        };
        x
    }
}

impl<TM: Send + Sync + Clone + 'static> Splitter<TM> {
    pub fn new(rect: Rect, app: &ApplicationPtr<TM>, vertical: bool) -> Splitter<TM> {
        Group::set_current(None::<&Group>);
        let mut s = Splitter {
            tile: Tile::new(rect.x, rect.y, rect.w, rect.h, None),
            app: app.clone(),
            panes: Rc::default(),
            vertical,
        };
        s.tile.set_frame(FrameType::DownBox);
        s.tile.end();
        let dx = 20;
        let dy = 20;
        let r = Frame::new(
            s.tile.x() + dx,
            s.tile.y() + dy,
            s.tile.w() - 2 * dx,
            s.tile.h() - 2 * dy,
            None,
        );
        s.tile.resizable(&r);
        s
    }

    pub fn add<W>(&mut self, w: &W)
    where
        W: Control<TM> + Clone + 'static,
    {
        self.panes.deref().borrow_mut().push(Box::new(w.clone()));
        let kidcount = self.panes.deref().borrow().len();
        if self.vertical {
            let childwidth = self.tile.w() / (kidcount) as i32;
            let lastw = self.tile.w() - childwidth * kidcount as i32;
            for (i, p) in self.panes.deref().borrow_mut().iter_mut().enumerate() {
                p.fl_widget().resize(
                    self.tile.x() + (i as i32 * childwidth),
                    self.tile.y(),
                    childwidth + if i == kidcount - 1 { lastw } else { 0 },
                    self.tile.h(),
                );
            }
        } else {
            let childheight = self.tile.h() / (kidcount) as i32;
            let lasth = self.tile.h() - childheight * kidcount as i32;
            for (i, p) in self.panes.deref().borrow_mut().iter_mut().enumerate() {
                p.fl_widget().resize(
                    self.tile.x(),
                    self.tile.y() + (i as i32 * childheight),
                    self.tile.w(),
                    childheight + if i == kidcount - 1 { lasth } else { 0 },
                );
            }
        }

        self.tile.add(&w.fl_widget());
    }
}

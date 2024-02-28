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
    split_percentages: Vec<i32>,
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
            split_percentages: vec![],
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

    pub fn add<W>(&mut self, w: &W, percentage: i32)
    where
        W: Control<TM> + Clone + 'static,
    {
        self.panes.deref().borrow_mut().push(Box::new(w.clone()));
        self.split_percentages.push(percentage);
        let kidcount = self.panes.deref().borrow().len();
        if self.vertical {
            let mut totw = 0;
            for (i, p) in self.panes.deref().borrow_mut().iter_mut().enumerate() {
                let mut cwidth = (self.split_percentages[i] * self.tile.w()) / 100;
                let cx = totw;
                // last child gets all the remianing space
                if i == kidcount - 1 {
                    cwidth = self.tile.w() - totw
                };
                totw += cwidth;
                p.fl_widget()
                    .resize(self.tile.x() + cx, self.tile.y(), cwidth, self.tile.h());
                self.tile.size_range_by_index(i as i32, 20, 20, 20, 20);
            }
        } else {
            let mut toth = 0;
            for (i, p) in self.panes.deref().borrow_mut().iter_mut().enumerate() {
                let mut cheight = (self.split_percentages[i] * self.tile.h()) / 100;
                let cy = toth;
                if i == kidcount - 1 {
                    cheight = self.tile.h() - toth
                };
                toth += cheight;
                p.fl_widget()
                    .resize(self.tile.x(), self.tile.y() + cy, self.tile.w(), cheight);
                self.tile.size_range_by_index(i as i32, 20, 20, 20, 20);
            }
        }
        self.tile.add(&w.fl_widget());
    }
}

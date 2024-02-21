// For a simpler boilerplate-less table, check the fltk-table crate

use std::{cell::RefCell, rc::Rc, sync::Arc};

use fltk::{
    app::Sender,
    draw::Rect,
    enums::{Color, Event, Font},
    group::Group,
    prelude::*,
    table::{TableRow, TableRowSelectMode},
    widget::Widget,
    *,
};

use super::{
    application::{ApplicationExt, ApplicationPtr},
    control::Control,
    misc::Theme,
};

//use crate::cxapp::Message;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColumnDefinition {
    pub name: String,
    pub width: i32,
    // pub resizable: bool,
    //  pub  sortable: bool,

    //  pub  tag: Option<String>
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Row {
    pub cells: Vec<String>,
    pub tag: Option<String>,
}
#[derive(Clone)]
pub struct ListBox<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    table: TableRow,
    rows: Arc<RefCell<Vec<Row>>>,
    cols: Rc<Vec<ColumnDefinition>>,
    app: ApplicationPtr<TM>,
    // theme: &'static Theme,
    phantom: std::marker::PhantomData<TM>,
}

impl<TM> ListBox<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    pub fn new(size: Rect, cols: &[ColumnDefinition], app: &ApplicationPtr<TM>) -> Self {
        Group::set_current(None::<&Group>);
        let mut table = TableRow::new(size.x, size.y, size.w, size.h, None)
            .with_type(TableRowSelectMode::Single);
        table.handle(|m, e| {
            println!("table event: {:?}, {:?},{:08x}", m, e, e.bits());
            true
        });
        table.set_rows(0);
        table.set_row_header(false);
        table.set_row_resize(false);
        table.set_cols(cols.len() as i32);
        table.set_col_header(true);
        table.set_color(app.get_theme().bg);
        table.set_col_header_height(25);
        table.set_col_header_color(app.get_theme().frame_color);
        table.set_row_height_all(20);
        table.set_col_resize(true);
        table.set_callback(move |t| {
            let (rt, _ct, _rb, _cb) = t.get_selection();
            println!("table callback: {:?}", rt);
        });

        for (i, col) in cols.iter().enumerate() {
            table.set_col_width(i as i32, col.width);
        }
        let mut s = ListBox {
            table,
            rows: Arc::default(),
            cols: Rc::new(cols.to_vec()),
            phantom: std::marker::PhantomData,
            app: app.clone(),
        };

        s.table.draw_cell({
            let cols = s.cols.clone();
            let rows = s.rows.clone();
            let theme = s.app.get_theme().clone();
            move  |t, ctx, row, col, x, y, w, h| match ctx {
            table::TableContext::StartPage => {
                draw::set_font( theme.font, theme.font_size as i32);
            },
            table::TableContext::ColHeader => {
                Self::draw_header(&cols,col, x, y, w, h,  &theme)
            }

            table::TableContext::Cell => {
                let cell = rows.borrow().get(row as usize).unwrap().cells.get(col as usize).unwrap().to_string();
                let (rt,_,_,_) = t.get_selection();
                let sel = rt == row;
                    Self::draw_data(
                &cell.as_str(),
                x,
                y,
                w,
                h,
                sel,
                &theme
            )}
            , // Data in cells
            _ => (),
        }});
        s.table.end();
        s
    }

    pub fn redraw(&mut self) {
        self.table.redraw();
    }
    pub fn set_rect(&mut self, r: Rect) {
        self.table.resize(r.x, r.y, r.w, r.h);
    }
    pub fn set_height(&mut self, h: i32) {
        self.table
            .resize(self.table.x(), self.table.y(), self.table.w(), h);
    }
    pub fn hide(&mut self) {
        self.table.hide();
    }
    pub fn show(&mut self) {
        self.table.show();
    }
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.table.x(),
            y: self.table.y(),
            w: self.table.w(),
            h: self.table.h(),
        }
    }
    pub fn add_row(&mut self, row: Row) {
        self.rows.borrow_mut().push(row);
        self.table.set_rows(self.rows.borrow_mut().len() as i32);
    }
    pub fn clear(&mut self) {
        self.rows.borrow_mut().clear();
    }
    // pub fn emit(&mut self, channel: Sender<Message>, msg: Message) {
    //     self.table.emit(channel, msg);
    // }
    pub fn selected_row(&mut self) -> Option<i32> {
        let (rt, _ct, _rb, _cb) = self.table.get_selection();

        Some(rt)
    }
    pub fn get_row(&self, row: i32) -> Option<Row> {
        let b = self.rows.borrow();
        let r = b[row as usize].clone();
        Some(r)
    }
    fn draw_header(
        cols: &Rc<Vec<ColumnDefinition>>,
        col: i32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        theme: &Theme,
    ) {
        draw::push_clip(x, y, w, h);
        // draw::draw_box(enums::FrameType::ThinUpBox, x, y, w, h, theme.bg);
        let txt = cols[col as usize].name.as_str();
        // draw::set_draw_color(theme.fg);
        draw::set_draw_color(theme.bg);
        draw::draw_rectf(x, y, w, h);
        draw::set_draw_color(theme.fg);
        draw::set_font(theme.font, theme.font_size as i32);
        draw::draw_text2(txt, x + 2, y, w, h, enums::Align::Left);

        if col > 0 {
            draw::draw_line(x, y, x, y + h)
        };
        if col == (cols.len() - 1) as i32 {
            draw::draw_line(x + w - 1, y, (x + w) - 1, y + h)
        };
        draw::pop_clip();
    }
    fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool, theme: &Theme) {
        draw::push_clip(x, y, w, h);
        //   println!("draw_data: {}, {}, {}, {}", x, y, w, h);
        if selected {
            draw::set_draw_color(theme.hl);
        } else {
            draw::set_draw_color(theme.bg);
        }
        draw::draw_rectf(x, y, w, h);
        draw::set_draw_color(theme.fg);
        draw::set_font(theme.font, theme.font_size as i32);
        draw::draw_text2(txt, x + 2, y, w, h, enums::Align::Left);
        //  draw::draw_rect(x, y, w, h);
        draw::pop_clip();
    }
}
impl<TM> Control<TM> for ListBox<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    fn fl_widget(&self) -> Widget {
        let x = unsafe {
            Widget::from_widget_ptr(self.table.as_widget_ptr() as *mut fltk_sys::widget::Fl_Widget)
        };
        x
    }
}

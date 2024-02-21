// big chunks copied from fltk-rs tree sample

use std::{cell::RefCell, rc::Rc};

use fltk::{
    app,
    draw::Rect,
    enums::{Color, Event},
    prelude::{FltkError, WidgetBase, WidgetExt},
    tree::{Tree, TreeItem, TreeReason},
    widget::Widget,
};

use super::{
    application::{ApplicationExt, ApplicationPtr},
    control::Control,
};
#[derive(Clone)]
pub struct TreeView<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    app: ApplicationPtr<TM>,

    tree: fltk::tree::Tree,
    previous_focus: Rc<RefCell<Option<TreeItem>>>,
}
#[derive(PartialEq)]
enum State {
    MovingUp,
    MovingDown,
    Undefined,
}

fn verify_open_till_root(opt: &Option<fltk::tree::TreeItem>) -> bool {
    let mut par = opt.clone();
    loop {
        match par.as_ref().unwrap().parent() {
            Some(p) => {
                if p.is_close() {
                    return false;
                } else {
                    par = Some(p.clone());
                }
            }
            None => return true,
        }
    }
}
impl<TM: Send + Sync + Clone + 'static> TreeView<TM> {
    pub fn new(rect: Rect, app: &ApplicationPtr<TM>) -> Self {
        let mut tree = Tree::new(rect.x, rect.y, rect.w, rect.h, None);
        let previous_focus = Rc::new(RefCell::new(None::<TreeItem>));
        let theme = app.get_theme();
        let pfr = Rc::clone(&previous_focus);
        let mut root = TreeItem::new(&tree, "Segments");
        root.set_label_bgcolor(Color::Blue);
        root.set_label_color(Color::White);
        root.set_label_fgcolor(Color::Red);
        root.set_label_font(theme.font);
        root.set_label_size(theme.font_size);
        tree.set_root(Some(root));
        tree.set_selection_color(theme.hl);
        tree.set_color(theme.bg);
        tree.set_item_label_fgcolor(theme.fg);
        tree.set_item_label_font(theme.font);
        tree.set_item_label_size(theme.font_size);
        //tree.set_root_label("Segments");

        //tree.set_show_root(false);
        tree.set_callback_reason(TreeReason::Selected);
        tree.set_callback({
            let sender = app.get_sender().clone();
            move |t| {
                println!("clicked an item");
                if let Some(sel) = t.get_selected_items() {
                    if let Some(item) = sel.iter().next() {
                        println!("Selected item: {:?}", item.label());
                        if let Some(msg) = unsafe { item.user_data::<TM>() } {
                            sender.send(msg);
                        }
                    }
                }
            }
        });

        tree.handle(move |t, e| match e {
            Event::Move => {
                let (_, mouse_y) = app::event_coords();
                let mut state = State::Undefined;
                let mut pf = pfr.borrow_mut();
                loop {
                    match &*pf {
                        Some(item) => {
                            let item_y = item.y();
                            match state {
                                State::MovingUp => {
                                    if verify_open_till_root(&pf) {
                                        if mouse_y < item_y {
                                            *pf = pf.as_ref().unwrap().prev();
                                            continue;
                                        };
                                        break;
                                    } else {
                                        *pf = pf.as_ref().unwrap().prev();
                                        continue;
                                    }
                                }
                                State::MovingDown => {
                                    if verify_open_till_root(&pf) {
                                        if mouse_y > item_y + item.h() {
                                            *pf = pf.as_ref().unwrap().next();
                                            continue;
                                        };
                                        break;
                                    } else {
                                        *pf = pf.as_ref().unwrap().next();
                                        continue;
                                    }
                                }
                                State::Undefined => {
                                    if mouse_y < item_y {
                                        *pf = pf.as_ref().unwrap().prev();
                                        state = State::MovingUp;
                                        continue;
                                    };
                                    if mouse_y > item_y + item.h() {
                                        *pf = pf.as_ref().unwrap().next();
                                        state = State::MovingDown;
                                        continue;
                                    };
                                    return true; // If in same range, don't update 'previous_focus'
                                }
                            }
                        }
                        // End up here if y is outside tree boundaries, or no tree item is present
                        None => match &state {
                            State::MovingUp | State::MovingDown => return true,
                            State::Undefined => {
                                *pf = t.first();
                                state = State::MovingDown;
                                if pf.is_none() {
                                    return true;
                                }
                                continue;
                            }
                        },
                    };
                }
                if verify_open_till_root(&pf) {
                    t.take_focus().ok();
                    t.set_item_focus(pf.as_ref().unwrap());
                    println!("Set focus to item: {:?}", pf.as_ref().unwrap().label());
                }
                true
            }
            _ => false,
        });
        Self {
            tree,
            previous_focus,
            app: app.clone(),
        }
    }

    pub fn add(&mut self, path: &str, message: TM) -> Option<TreeItem> {
        let mut item = self.tree.add(path);
        if let Some(ref mut ti) = item {
            ti.set_user_data(message);
        }
        item
    }

    /// Caution, variable 'previous focus' must be set to None, as it
    /// otherwise could try to refer to an already freed memory location,
    /// when this TreeItem is removed.
    pub fn remove(&mut self, item: &TreeItem) -> Result<(), FltkError> {
        *self.previous_focus.borrow_mut() = None;
        self.tree.remove(item)
    }

    pub fn get_items(&self) -> Option<Vec<TreeItem>> {
        self.tree.get_items()
    }
}

impl<TM> Control<TM> for TreeView<TM>
where
    TM: Send + Sync + Clone + 'static,
{
    fn fl_widget(&self) -> Widget {
        let x = unsafe {
            Widget::from_widget_ptr(self.tree.as_widget_ptr() as *mut fltk_sys::widget::Fl_Widget)
        };
        x
    }
}

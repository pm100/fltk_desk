use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use fltk::app::{self, Receiver, Sender};

use super::misc::Theme;
pub struct Application<T: Send + Sync> {
    app: app::App,
    sender: Sender<T>,
    receiver: Receiver<T>,
    theme: Theme,
}
pub type ApplicationPtr<TM> = Rc<RefCell<Application<TM>>>;
pub trait ApplicationExt<T: Send + Sync> {
    fn get_theme(&self) -> Theme;
    fn get_sender(&self) -> Sender<T>;
    fn run(&self, dispatch: impl FnMut(T) -> bool);
    fn quit(&self);
}
impl<T: Send + Sync + 'static> ApplicationExt<T> for Rc<RefCell<Application<T>>> {
    fn get_theme(&self) -> Theme {
        self.borrow().theme
    }

    fn get_sender(&self) -> Sender<T> {
        self.borrow().sender.clone()
    }

    fn run(&self, dispatch: impl FnMut(T) -> bool) {
        self.borrow().run(dispatch)
    }

    fn quit(&self) {
        todo!()
    }
}
impl<T: 'static + Send + Sync> Application<T> {
    pub fn new(theme: Theme) -> ApplicationPtr<T> {
        let app = app::App::default().with_scheme(app::Scheme::Gtk);
        let (sender, receiver) = app::channel::<T>();
        Rc::new(RefCell::new(Application {
            app,
            sender,
            receiver,
            theme,
        }))
    }
    pub fn get_sender(&self) -> Sender<T> {
        self.sender.clone()
    }
    pub fn run(&self, mut dispatch: impl FnMut(T) -> bool) {
        while self.app.wait() {
            if let Some(msg) = self.receiver.recv() {
                if dispatch(msg) {
                    break;
                }
            }
        }
    }
    pub fn quit(&self) {}
}

use fltk::{
    app::Sender,
    draw::Rect,
    enums::{Color, Event, Font, FrameType, Shortcut},
    prelude::*,
    *,
};

use fltk_desk::ui::{
    application::Application,
    control::Control,
    listbox::{ColumnDefinition, ListBox, Row},
    menu::Menu,
    misc::{Environment, Theme},
    splitter::{self, Splitter},
    textbox::TextBox,
};

#[derive(Copy, Clone)]
enum Message {
    Quit = 0,
    MenuEditCut = 1,
    LoadBinary = 2,
}
impl Into<u64> for Message {
    fn into(self) -> u64 {
        self as u64
    }
}
impl From<u64> for Message {
    fn from(val: u64) -> Self {
        match val {
            0 => Message::Quit,
            1 => Message::MenuEditCut,
            2 => Message::LoadBinary,
            _ => panic!("Invalid message"),
        }
    }
}
fn main() {
    // let app = app::App::default().with_scheme(app::Scheme::Gleam);
    // let mut wind = window::Window::default().with_size(800, 600);

    // wind.make_resizable(true);

    let app = Application::new();
    let window = MainWin::new(
        Rect {
            x: 0,
            y: 0,
            w: 800,
            h: 600,
        },
        env,
    );

    //  main_win.set_color(Color::White);
    let theme: &'static Theme = &Theme {
        bg: Color::Black,
        fg: Color::White,
        hl: Color::Red,
        frame_color: Color::Dark2,
        font: Font::Helvetica,
        font_size: 14,
        mono_font: Font::Courier,
        mono_font_size: 14,
        popbg: Color::White,
    };
    let theme2: &'static Theme = &Theme {
        bg: Color::White,
        fg: Color::Black,
        hl: Color::Red,
        frame_color: Color::Dark2,
        font: Font::Helvetica,
        font_size: 14,
        mono_font: Font::Courier,
        mono_font_size: 14,
        popbg: Color::Gray0,
    };
    let (s, r) = app::channel::<u64>();
    let env = &Environment { theme, channel: s };
    let menu = MainMenu::new(&env);
    let mut vsplitter = Splitter::new(
        Rect {
            x: 0,
            y: 34,
            w: 800,
            h: 600 - 34,
        },
        env,
        true,
    );
    let mut hsplitter = Splitter::new(
        Rect {
            x: 0,
            y: 34,
            w: 800,
            h: 600 - 34,
        },
        env,
        false,
    );
    let cols = vec![
        ColumnDefinition {
            name: "Name".to_string(),
            width: 100,
        },
        ColumnDefinition {
            name: "Start".to_string(),
            width: 40,
        },
        ColumnDefinition {
            name: "Size".to_string(),
            width: 40,
        },
    ];
    let mut seglist: ListBox = ListBox::new(
        Rect {
            x: 0,
            y: 34,
            w: 400,
            h: (600 - 34) / 2,
        },
        &cols,
        env,
    );
    let cols = vec![
        ColumnDefinition {
            name: "Name".to_string(),
            width: 100,
        },
        ColumnDefinition {
            name: "Start".to_string(),
            width: 40,
        },
        ColumnDefinition {
            name: "Size".to_string(),
            width: 40,
        },
    ];

    let r1 = Row {
        cells: vec!["Name".to_string(), "Start".to_string(), "Size".to_string()],
        tag: None,
    };
    let r2 = Row {
        cells: vec!["yo".to_string(), "1".to_string(), "2".to_string()],
        tag: None,
    };
    seglist.add_row(r1);
    seglist.add_row(r2);
    let mut symlist: ListBox = ListBox::new(
        Rect {
            x: 0,
            y: 34,
            w: 400,
            h: (600 - 34) / 2,
        },
        &cols,
        env,
    );
    hsplitter.add(seglist);
    hsplitter.add(symlist);
    vsplitter.add(hsplitter);
    let mut textbox = TextBox::new(
        Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 600,
        },
        theme,
    );
    textbox.append("yo: 1\n");
    vsplitter.add(textbox);
    wind.add(&vsplitter.fl_widget());
    app.run(dispatch_message);
    fn dispatch_message(msg: u64) -> bool {
        let msg = msg as Message;
        return match msg {
            Message::Quit => true,
            Message::LoadBinary => {
                println!("Load binary");
                false
            }
            _ => false,
        };
    }
}
pub struct MainMenu {
    menu: Menu,
}
fn menu_cb(m: &mut impl MenuExt) {
    if let Ok(mpath) = m.item_pathname(None) {
        println!("Menu '{}'", mpath);
    }
}

impl MainMenu {
    pub fn new(env: &Environment) -> Self {
        let mut menu = Menu::new(env);

        menu.add(
            "&File/   Open...\t",
            Shortcut::Ctrl | 'o',
            Message::LoadBinary.into(),
        );

        // menu.add(
        //     "&File/   Save\t",
        //     Shortcut::Ctrl | 's',
        //     menu::MenuFlag::Normal,
        //     menu_cb,
        // );

        // menu.add(
        //     "&File/   Save as...\t",
        //     Shortcut::Ctrl | 'w',
        //     menu::MenuFlag::Normal,
        //     menu_cb,
        // );

        // menu.add(
        //     "&File/   Print...\t",
        //     Shortcut::Ctrl | 'p',
        //     menu::MenuFlag::MenuDivider,
        //     menu_cb,
        // );

        // menu.add_emit(
        //     "&File/   Quit\t",
        //     Shortcut::Ctrl | 'q',
        //     menu::MenuFlag::Normal,
        //     channel,
        //     Message::Quit,
        // );

        // menu.add_emit(
        //     "&Edit/   Cut\t",
        //     Shortcut::Ctrl | 'x',
        //     menu::MenuFlag::Normal,
        //     channel,
        //     Message::MenuEditCut,
        // );

        // menu.add(
        //     "&Edit/Copy\t",
        //     Shortcut::Ctrl | 'c',
        //     menu::MenuFlag::Normal,
        //     menu_cb,
        // );

        // menu.add(
        //     "&Edit/Paste\t",
        //     Shortcut::Ctrl | 'v',
        //     menu::MenuFlag::Normal,
        //     menu_cb,
        // );

        // menu.add(
        //     "&Help/About\t",
        //     Shortcut::None,
        //     menu::MenuFlag::Normal,
        //     menu_cb,
        // );

        Self { menu }
    }
}

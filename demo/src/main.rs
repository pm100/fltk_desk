use fltk::{
    app::Sender,
    draw::Rect,
    enums::{Color, Event, Font, FrameType, Shortcut},
    prelude::*,
    *,
};

use fltk_desk::ui::{
    control::Control,
    listbox::{ColumnDefinition, ListBox},
    misc::Theme,
    splitter::{self, Splitter},
    textbox::TextBox,
};

#[derive(Copy, Clone)]
pub enum Message {
    Quit,
    LoadBinary,
    SegListPick,
    MenuEditCut,
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = window::Window::default().with_size(800, 600);

    wind.make_resizable(true);

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
    };
    let theme: &'static Theme = &Theme {
        bg: Color::White,
        fg: Color::Black,
        hl: Color::Red,
        frame_color: Color::Dark2,
        font: Font::Helvetica,
        font_size: 14,
        mono_font: Font::Courier,
        mono_font_size: 14,
    };
    let (s, r) = app::channel::<Message>();
    let menu = MainMenu::new(&s, theme);
    let mut vsplitter = Splitter::new(
        Rect {
            x: 0,
            y: 34,
            w: 800,
            h: 600 - 34,
        },
        theme,
        true,
    );
    let mut hsplitter = Splitter::new(
        Rect {
            x: 0,
            y: 34,
            w: 800,
            h: 600 - 34,
        },
        theme,
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
        theme,
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
    let mut symlist: ListBox = ListBox::new(
        Rect {
            x: 0,
            y: 34,
            w: 400,
            h: (600 - 34) / 2,
        },
        &cols,
        theme,
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
    wind.show();
    app.run();
}
pub struct MainMenu {
    menu: menu::SysMenuBar,
}
fn menu_cb(m: &mut impl MenuExt) {
    if let Ok(mpath) = m.item_pathname(None) {
        println!("Menu '{}'", mpath);
    }
}

impl MainMenu {
    pub fn new(channel: &app::Sender<Message>, theme: &Theme) -> Self {
        let mut menu = menu::SysMenuBar::default().with_size(800, 34);
        menu.set_color(theme.bg);
        menu.set_text_color(theme.fg);
        menu.set_frame(FrameType::FlatBox);
        menu.add_emit(
            "&File/Load Binary..\t",
            Shortcut::Ctrl | 'l',
            menu::MenuFlag::Normal,
            *channel,
            Message::LoadBinary,
        );

        menu.add(
            "&File/Open...\t",
            Shortcut::Ctrl | 'o',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&File/Save\t",
            Shortcut::Ctrl | 's',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&File/Save as...\t",
            Shortcut::Ctrl | 'w',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&File/Print...\t",
            Shortcut::Ctrl | 'p',
            menu::MenuFlag::MenuDivider,
            menu_cb,
        );

        menu.add_emit(
            "&File/Quit\t",
            Shortcut::Ctrl | 'q',
            menu::MenuFlag::Normal,
            *channel,
            Message::Quit,
        );

        menu.add_emit(
            "&Edit/Cut\t",
            Shortcut::Ctrl | 'x',
            menu::MenuFlag::Normal,
            *channel,
            Message::MenuEditCut,
        );

        menu.add(
            "&Edit/Copy\t",
            Shortcut::Ctrl | 'c',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&Edit/Paste\t",
            Shortcut::Ctrl | 'v',
            menu::MenuFlag::Normal,
            menu_cb,
        );

        menu.add(
            "&Help/About\t",
            Shortcut::None,
            menu::MenuFlag::Normal,
            menu_cb,
        );

        Self { menu }
    }
}

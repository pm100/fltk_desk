use fltk::{
    draw::Rect,
    enums::{Color, Font, Shortcut},
};
use fltk_desk::ui::{
    application::{Application, ApplicationExt},
    listbox::{ColumnDefinition, ListBox, Row},
    mainwin::MainWin,
    menu::Menu,
    misc::Theme,
    splitter::Splitter,
    textbox::TextBox,
    treeview::TreeView,
};
#[derive(Copy, Clone, Debug)]
enum Message {
    Quit = 0,
    MenuEditCut = 1,
    LoadBinary = 2,
}
fn main() {
    pub const MyDark1: Color = Color::from_rgbi(34);
    let theme = Theme {
        bg: Color::Black,
        fg: Color::White,
        hl: Color::from_rgbi(35),
        frame_color: Color::Dark2,
        font: Font::Helvetica,
        font_size: 14,
        mono_font: Font::Courier,
        mono_font_size: 14,
        popbg: MyDark1,
    };

    let theme2 = Theme {
        bg: Color::White,
        fg: Color::Black,
        hl: Color::from_rgbi(41),
        frame_color: Color::Dark2,
        font: Font::Helvetica,
        font_size: 14,
        mono_font: Font::Courier,
        mono_font_size: 14,
        popbg: Color::from_rgbi(48),
    };
    let app = Application::<Message>::new(theme2);
    let mut menu = Menu::new(&app);
    let mut window = MainWin::<Message>::new(
        Rect {
            x: 0,
            y: 0,
            w: 800,
            h: 600,
        },
        &app,
    );
    menu.add(
        "&File/   Open...\t",
        Shortcut::Ctrl | 'o',
        Message::LoadBinary,
    );
    menu.add("&File/   Quit...\t", Shortcut::Ctrl | 'c', Message::Quit);
    window.add(&menu);
    window.end();
    let mut vsplitter = Splitter::<Message>::new(
        Rect {
            x: 0,
            y: 34,
            w: 800,
            h: 600 - 34,
        },
        &app,
        true,
    );
    let mut hsplitter = Splitter::<Message>::new(
        Rect {
            x: 0,
            y: 34,
            w: 800,
            h: 600 - 34,
        },
        &app,
        false,
    );
    window.add(&vsplitter);
    // window.add(&hsplitter);
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
    let mut seglist = ListBox::<Message>::new(
        Rect {
            x: 0,
            y: 34,
            w: 400,
            h: (600 - 34) / 2,
        },
        &cols,
        &app,
    );

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
    let mut symlist = ListBox::<Message>::new(
        Rect {
            x: 0,
            y: 34,
            w: 400,
            h: (600 - 34) / 2,
        },
        &cols,
        &app,
    );
    let mut textbox = TextBox::<Message>::new(
        Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 600,
        },
        &app,
    );
    textbox.append("yo: 1\n");

    hsplitter.add(&seglist);
    hsplitter.add(&symlist);
    vsplitter.add(&hsplitter);
    vsplitter.add(&textbox);
    let mut tree = TreeView::<Message>::new(
        Rect {
            x: 0,
            y: 0,
            w: 400,
            h: 600,
        },
        &app,
    );
    tree.add("a/b", Message::LoadBinary);
    tree.add("a/c", Message::MenuEditCut);
    vsplitter.add(&tree);
    window.show();
    app.run(|msg| {
        println!("Received message: {:?}", msg);
        match msg {
            Message::Quit => true,
            _ => false,
        }
    });
}

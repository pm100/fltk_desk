use fltk::{
    app,
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
        faint_text: Color::Dark3,
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
        faint_text: Color::from_rgbi(45),
    };
    //let mut vcol = vec![];
    // for i in 32..56 {
    //     let c = Color::from_rgbi(i);
    //     let rgb = c.to_rgb();
    //     let oldc = Color::from_rgbi(32 + (56 - i));
    //     println!("Color: {:?}, {:?}, {:?}, {:?}", oldc, rgb.0, rgb.1, rgb.2);
    //     app::set_color(oldc, rgb.0, rgb.1, rgb.2);
    // }
    // app::set_color(Color::Black, 255, 255, 255);
    // app::set_color(Color::White, 0, 0, 0);

    let app = Application::<Message>::new(theme2);
    let height = if cfg!(target_os = "macos") { 0 } else { 34 };
    println!("{}", height);
    let mut menu = Menu::new(&app, height);
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
            y: height,
            w: 800,
            h: 600 - height,
        },
        &app,
        true,
    );
    let mut hsplitter = Splitter::<Message>::new(
        Rect {
            x: 0,
            y: height,
            w: 800,
            h: 600 - height,
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
            y: 0,
            w: 0,
            h: 0,
        },
        &cols,
        &app,
    );

    let r1 = Row {
        cells: vec!["foo".to_string(), "wizz".to_string(), "bang".to_string()],
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
            y: 0,
            w: 0,
            h: 0,
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

    hsplitter.add(&seglist, 60);
    hsplitter.add(&symlist, 40);
    vsplitter.add(&hsplitter, 20);
    vsplitter.add(&textbox, 30);
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
    vsplitter.add(&tree, 0);
    window.show();
    app.run(|msg| {
        println!("Received message: {:?}", msg);
        match msg {
            Message::Quit => true,
            Message::LoadBinary => {
                fltk::dialog::alert(center().0 - 200, center().1 - 100, "Please specify a file!");
                false
            }
            _ => false,
        }
    });
}
pub fn center() -> (i32, i32) {
    (
        (fltk::app::screen_size().0 / 2.0) as i32,
        (fltk::app::screen_size().1 / 2.0) as i32,
    )
}

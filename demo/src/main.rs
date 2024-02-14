use fltk::{
    app::Sender,
    draw::Rect,
    enums::{Color, Event, Font},
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
    let mut vsplitter = Splitter::new(
        Rect {
            x: 0,
            y: 0,
            w: 800,
            h: 600,
        },
        theme,
        true,
    );
    let mut hsplitter = Splitter::new(
        Rect {
            x: 0,
            y: 0,
            w: 800,
            h: 600,
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

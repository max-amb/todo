use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use cursive::theme::{BorderStyle, Palette};
use cursive::Cursive;
use cursive::traits::Nameable;
use cursive::views::*;
use cursive::traits::*;
use crate::delete::gui_delete;
use crate::add::gui_add;
use crate::generate_path;
use crate::list::gui_list;

pub fn run_gui() {
    let mut cursive_root = cursive::default();
    let tasks = Dialog::around(SelectView::<String>::new()
               .on_submit(gui_delete)
               .with_name("tasks")
               .scrollable()).title("Tasks").full_height();

    let add_task = Dialog::around(EditView::new().
        on_submit(gui_add)
        .with_name("add_task")).full_width();

    let quit_button = Button::new("Quit", on_quit);
    let global_options = LinearLayout::vertical()
        .child(Button::new("Light", light_theming))
        .child(Button::new("Dark", dark_theming))
        .child(quit_button);

    let h_layout = LinearLayout::horizontal()
        .child(add_task)
        .child(global_options);

    let v_layout = LinearLayout::vertical()
        .child(tasks)
        .child(h_layout)
        .full_screen();
    cursive_root.add_layer(v_layout);

    let quit_warning = Dialog::info("To quit the program ensure you use the quit button").title("IMPORTANT");
    cursive_root.add_layer(quit_warning);
    gui_list(&mut cursive_root);
    light_theming(&mut cursive_root);
    cursive_root.run();
}

fn on_quit(siv: &mut Cursive) {
    siv.call_on_name("tasks", |view: &mut SelectView::<String>| {
        let items: Vec<String> = view.iter().map(|x| x.1.to_string()).collect();
        let mut file_opened = opening().unwrap();
        items.into_iter().for_each(|f| _ = file_opened.write((f+"\n").as_bytes()).expect("Write failed"));
    });
    siv.quit()
}

fn opening() -> Result<File, Error> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(generate_path())
        .expect("failed to create/add to file");
    return Ok(file);
}

fn dark_theming(siv: &mut Cursive) {
    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                use cursive::theme::Color::TerminalDefault;
                use cursive::theme::PaletteColor::*;

                palette[Background] = Red.light();
                palette[View] = TerminalDefault;
                palette[Primary] = White.dark();
                palette[TitlePrimary] = White.light();
                palette[Secondary] = Red.dark();
                palette[Highlight] = Red.dark();
                palette[HighlightInactive] = Red.light();
            }
        }),
    })
}

fn light_theming(siv: &mut Cursive) {
    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: Palette::default().with(|palette| {
            use cursive::theme::BaseColor::*;
            {
                use cursive::theme::PaletteColor::*;

                palette[Background] = Blue.light();
                palette[View] = White.dark();
                palette[Primary] = Black.dark();
                palette[TitlePrimary] = Black.dark();
                palette[Secondary] = Blue.dark();
                palette[Highlight] = Blue.dark();
                palette[HighlightInactive] = Blue.dark();
            }
        }),
    })
}
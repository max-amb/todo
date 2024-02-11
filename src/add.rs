use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use cursive::Cursive;
use cursive::views::{SelectView, EditView};
use crate::generate_path;

pub fn console_add(input: String) {
    let file_opened = opening();
    file_opened.unwrap().write((input+"\n").as_bytes()).expect("Write failed");
}

fn opening() -> Result<File, Error> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(generate_path())
        .expect("failed to create/add to file");
    return Ok(file);
}

pub fn gui_add(siv: &mut Cursive, input: &str) {
    siv.call_on_name("tasks", |view: &mut SelectView::<String>| {
        if !input.is_empty() {
            view.add_item_str(input.to_string());
        };
    });
    siv.call_on_name("add_task", |field: &mut EditView| {
        field.set_content("");
    });
}
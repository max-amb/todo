use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error};
use cursive::views::SelectView;
use cursive::Cursive;
use crate::generate_path;

pub fn console_list() {
    let file_opened = opening();
    for line in BufReader::new(file_opened.unwrap()).lines().enumerate() {
        println!("{} -> {}", line.0+1, line.1.unwrap());
    }
}

fn opening() -> Result<File, Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(generate_path())
        .expect("Failed to read tasks file");
    return Ok(file);
}

pub fn gui_list(siv: &mut Cursive) {
    let file_opened = opening();
    siv.call_on_name("tasks", |view: &mut SelectView::<String>| {
        view.add_all_str(BufReader::new(file_opened.unwrap()).lines().map(|x| x.unwrap()))
    });
}
use std::fs::{self, read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader, Error};
use cursive::Cursive;
use cursive::views::SelectView;
use crate::generate_path;

pub fn console_delete(task_id: i32) {
    let file_opened = opening();
    let lines = BufReader::new(file_opened.unwrap()).lines().enumerate()
        .filter(|x| (x.0 != (task_id-1) as usize))
        .map(|y| y.1.unwrap())
        .collect::<Vec<String>>().join("\n"); 
    if  read_to_string(generate_path()).unwrap() == lines || task_id == 0 {
        println!("Task does not exist");
        return;
    }
    fs::write(generate_path(), lines).expect("Failed to update tasks file");
}

fn opening() -> Result<File, Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(generate_path())
        .expect("tasks file does not exist - try adding a task");
    return Ok(file);
}

pub fn gui_delete(siv: &mut Cursive, _: &String) {
    siv.call_on_name("tasks", |view: &mut SelectView::<String>| {
        // println!("Selected -> {}", view.selected_id().unwrap());
        view.remove_item(view.selected_id().unwrap());
    });
}
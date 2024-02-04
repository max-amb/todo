use std::fs::{self, read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

pub fn delete(task_id: i32, path: PathBuf) {
    let file_opened = opening(&path);
    let lines = BufReader::new(file_opened.unwrap()).lines().enumerate()
        .filter(|x| (x.0 != task_id as usize))
        .map(|y| y.1.unwrap())
        .collect::<Vec<String>>().join("\n"); 
    if  read_to_string(&path).unwrap() == lines {
        println!("Task does not exist");
        return;
    }
    fs::write(path, lines).expect("Failed to update tasks file");
}

fn opening(path: &PathBuf) -> Result<File, Error> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("tasks file does not exist - try adding a task");
    return Ok(file);
}
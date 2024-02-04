use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error};
use std::path::PathBuf;

pub fn list(path: PathBuf) {
    let file_opened = opening(&path);
    for line in BufReader::new(file_opened.unwrap()).lines().enumerate() {
        println!("{} -> {}", line.0+1, line.1.unwrap());
    }
}

fn opening(path: &PathBuf) -> Result<File, Error> {
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Failed to read tasks file");
    return Ok(file);
}
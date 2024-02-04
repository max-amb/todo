use std::fs::{File, OpenOptions};
use std::io::{Error, Write};
use std::path::PathBuf;

pub fn add(input: String, path: PathBuf) {
    let file_opened = opening(&path);
    file_opened.unwrap().write((input+"\n").as_bytes()).expect("Write failed");
}

fn opening(path: &PathBuf) -> Result<File, Error> {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("failed to create/add to file");
    return Ok(file);
}
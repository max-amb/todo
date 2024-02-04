use std::env;
use std::fs::OpenOptions;
use std::io::Write;

pub fn add(input: String) {
    let mut x = env::current_exe().unwrap();
    x.pop();
    x.push("tasks");
    let file = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(x);
    file.unwrap().write(("\n".to_string()+&input).as_bytes()).expect("Write failed");
}
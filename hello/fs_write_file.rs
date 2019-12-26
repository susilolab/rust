use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open("hello.txt");

    match file {
        Err(e) => eprintln!("{}", e),
        Ok(mut f) => {
            f.write_all("Hello world lagu!.".to_owned().as_bytes());
        }
    }
}
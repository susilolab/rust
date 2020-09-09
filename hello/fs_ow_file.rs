// Buka file dan replace isi yang sudah ada.
//
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("hello.txt");

    match file {
        Err(e) => eprintln!("{}", e),
        Ok(mut f) => {
            let _ = f.write_all("Hello world lagu!.".to_owned().as_bytes());
        }
    }
}

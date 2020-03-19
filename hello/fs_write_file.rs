/// Buka file jika file tidak ada buat baru
/// jika file sudah ada tambah isinya ke pointer terakhir
///
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
            let _ = f.write_all("Hello world lagu!.".to_owned().as_bytes());
        }
    }
}
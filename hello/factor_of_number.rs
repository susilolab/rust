// use std::io;
use std::io::{self, Write};

fn main() {
	print!("Masukan angka yang akan dicari faktornya: ");
	let stdout = io::stdout();
	let mut handle = stdout.lock();
	let _ = handle.flush();

	let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let limit: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    for i in 1..=limit {
        if limit % i == 0 {
            println!("{}", i);
        }
    }
}

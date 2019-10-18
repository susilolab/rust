// use std::io;
use std::io::{self, Write};

fn main() {
	print!("Nama Anda: ");
	let stdout = io::stdout();
	let mut handle = stdout.lock();
	let _ = handle.flush();

	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	println!("{}", input);
}

use std::io;
use std::io::{Write};

fn main() {
	let mut input = String::new();
	print!("Ketik nama Kamu: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut input).unwrap();

	println!("Nama kamu: {}", input.trim());
}

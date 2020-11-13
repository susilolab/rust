use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let mut f = File::open("hello.rs")?;
	let mut buffer = [0; 5];

	let mut handle = f.take(5);
	handle.read(&mut buffer)?;

	println!("{:?}", buffer.iter().map(|&c| c as char).collect::<String>());

	Ok(())
}

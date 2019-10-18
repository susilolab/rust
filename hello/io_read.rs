use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
	let mut f = File::open("bil_prima.rs")?;
	let mut buffer = [0; 10];

	let n = f.read(&mut buffer)?;

	println!("The bytes: {:?}", &buffer[..n]);
	Ok(())
}

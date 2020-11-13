use std::io;
use std::io::Write;

fn main() -> std::io::Result<()> {
	print!("Ketikan nama kamu: ");
	io::stdout().flush()?;

	let mut input = String::new();
	io::stdin().read_line(&mut input)?;

	println!("Namamu: {}", input);

	Ok(())
}

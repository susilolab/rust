use std::fs;
use std::path::Path;

fn main() {
	let dir = Path::new("d:/Apps");
	for entry in fs::read_dir(dir).unwrap() {
		println!("{}", entry.unwrap().path().display());
	}

	let entr = fs::read_dir(dir)?;
}
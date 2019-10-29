use std::fs;

fn main() {
	for entry in fs::read_dir(".").unwrap() {
		let dir = entry.unwrap();
		println!("{:?}", dir.path());
	}
}
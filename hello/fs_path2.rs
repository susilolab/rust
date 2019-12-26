use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &Fn(&DirEntry)) -> io::Result<()> {
	if dir.is_dir() {
		for entry in fs::read_dir(dir)? {
			let entry = entry?;
			let path = entry.path();
			if path.is_dir() {
				visit_dirs(dir, cb)?;
			}else {
				cb(&entry);
			}
		}
	}
	Ok(())
}

fn main() {
	// for entry in fs::read_dir(".")? {
	// 	let dir = entry?;
	// 	println!("{:?}", dir.path());
	// }
}
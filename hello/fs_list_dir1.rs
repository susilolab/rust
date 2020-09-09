use std::fs;

fn main() {
	// paths mengembalikan iterator dari std::fs:DirEntry
	let paths = fs::read_dir("d:/Apps").unwrap();

	for path in paths {
		//                path.unwrap() mengembalikan PathBuf
		//                                 V
		println!("Name: {}", path.unwrap().path().display());
		//                                          ^
		//                              display() dari impl PathBuf
	}
}
use std::path::Path;

fn main() {
	let path = Path::new("/usr/bin");
	let fname = path.file_name().unwrap().to_str().unwrap();
	println!("{}", fname);
}

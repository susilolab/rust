use std::path::Path;

fn main() {
	let dir = Path::new("/tmp/hello.txt");
	assert_eq!("txt", dir.extension().unwrap());
}

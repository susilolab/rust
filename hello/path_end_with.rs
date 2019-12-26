use std::path::Path;

fn main() {
	let dir = Path::new("/tmp/passwd");
	assert!(dir.ends_with("passwd"));
}

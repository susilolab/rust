use std::path::Path;

fn main() {
	let dir = Path::new("/tmp");
	if dir.exists() {
		println!("folder ada!.");
	} else {
		println!("folder tidak ada!.");
	}
}
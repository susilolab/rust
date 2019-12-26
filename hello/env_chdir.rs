// Set direktori saat ini
//
// 8/12/2019 04:46
//
use std::env;
use std::path::Path;

fn main() {
	getcwd();

	let root = Path::new("/home/susilo");
	let _ = env::set_current_dir(&root);

	getcwd();
}

fn getcwd() {
	let path = env::current_dir().unwrap();
	println!("{}", path.display());
}

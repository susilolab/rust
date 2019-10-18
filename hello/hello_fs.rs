use std::fs;

fn main() {
	fs::create_dir("test1");
	fs::remove_dir_all("test1");
}
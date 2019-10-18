use std::fs;

fn main() {
	let data = "Hello world";
	fs::write("/tmp/foo", data).expect("Tidak dapat menulis file!");

	fs::rename("/tmp/foo", "/tmp/bar").unwrap();
}
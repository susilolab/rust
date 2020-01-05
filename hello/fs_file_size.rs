use std::fs;
use std::os::linux::fs::MetadataExt;

fn main() {
	let meta = fs::metadata(r#"d:\var\Rust\hello\ambil.rs"#).expect("error metadata");
	let file_size = meta.st_size();

	println!("{} bytes", file_size); // dalam byte
}

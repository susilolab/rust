use std::fs;
use std::os::windows::prelude::*;

fn main() {
	let metadata = fs::metadata("D:/Master/code--dive.mp4").unwrap();
	println!("{:?}", metadata.file_size());
}

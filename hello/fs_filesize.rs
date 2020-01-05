use std::fs;
use std::os::linux::fs::MetadataExt;

fn main() {
	let metadata = fs::metadata("D:/Master/code--dive.mp4").unwrap();
	println!("{:?}", metadata.st_size());
}

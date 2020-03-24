fn main() {
	let this_file: &str = file!();
	println!("file_name: {}", this_file);
	println!("line: {}", line!());
	println!("column: {}", column!());
}

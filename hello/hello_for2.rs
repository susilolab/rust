fn main() {
	let lines = "Hello\nWorld".lines();
	for (linenumber, line) in lines.enumerate() {
		println!("{}: {}", linenumber, line);
	}
}
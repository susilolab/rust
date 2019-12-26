fn find(haystack: &str, needle: char) -> Option<usize> {
	for (offset, c) in haystack.char_indices() {
		if c == needle {
			return Some(offset);
		}
	}
	None
}

fn main() {
	let file_name = "foobar";
	match find(file_name, '.') {
	    Some(i) => println!("File extension: {}", &file_name[i+1..]),
	    None => println!("No file extension found!."),
	}
}
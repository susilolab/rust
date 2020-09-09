fn main() {
	let word = "testing";
	let word_len = word.len();

	if word_len % 2 == 0 {
		let start = (word_len / 2) - 1;
		let end = word_len / 2;
		println!("{}", &word[start..=end]);
	
	} else {
		let start = word_len / 2;
		println!("{}", word.chars().nth(start).unwrap());
	}
}

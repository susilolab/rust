fn main() {
	assert!('A'.is_uppercase());

	let a = is_all_uppercase("HELLO?!");
	println!("{:?}", a);
}

fn is_all_uppercase(word: &str) -> bool {
	let mut res = true;

	for w in word.chars() {
		if w.is_alphabetic() && w.is_uppercase() {
			println!("#{}", w);
			res = res && true;
		} else if w.is_alphabetic() && !w.is_uppercase() {
			res = res && false;
			println!("~{}", w);
		}
	}

	res
}

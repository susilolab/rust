use std::collections::HashMap;

struct Translation {
	search: String,
	replace: String,
}

impl Translation {
	fn make_map(&mut self) -> HashMap<char, char> {
		let count = self.search.chars().count();
		let results = self.replace.chars().count();

		let mut keys = self.search.chars();
		for key in keys {
			println!("{:?}", key);
		}

		let mut values = self.replace.chars();
		for val in values {
			println!("{:?}", val);
		}
	}
}

fn main() {
}

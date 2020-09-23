use std::collections::HashMap;

fn main() {
	//                    p  s
	// [1, 2, 4, 4, 5, 6, 7, 3, 2, 7, 8, 9, 1]
	// [4, 5, 6, 7] -> len 4
	// [7, 8, 9] -> len = 3
	// jawaban = [4, 5, 6, 7]
	let s = [1, 2, 4, 4, 5, 6, 7, 3, 2, 7, 8, 9, 1];
	longest_incrementing_subslice(&s);
}

pub fn longest_incrementing_subslice(s: &[u8]) { //-> &[u8] {
	let mut p = s[0];
	let mut stack = Vec::new();
	let mut map = HashMap::new();
	let mut i = 0;

	for x in 1..s.len() {
		if s[x] > p {
			stack.push(s[x]);
		} else if s[x] == p || s[x] < p {
			map.insert(i, stack);
			i = i + 1;
			stack.clear();
		}
	}

	println!("{:#?}", map);
}
/*
s = [1, 2, 4, 4, 5, 6, 7, 3, 2, 7, 8, 9, 1]
p = 1
stack = [];

for i = 1; i < s.len(); i++ {
	s[i] = 3, p = 7
	if s[i] > p {
		stack.push(s[i]);
		p = s[i];
	} else if s[i] == p {
		stack.clear();

	} else if (p > s[i])
}


*/
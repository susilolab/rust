fn main() {
	let num = vec![1,2,3,4,5];

	let mut iter = (&num).into_iter();
	loop {
		let v = match iter.next() {
			Some(v) => v,
			None => break,
		};
		println!("{}", v);
	}
}
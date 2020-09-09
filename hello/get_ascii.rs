fn main() {
	let a = (b'A'..=b'Z').map(char::from);
	for x in a {
		println!("{}", x);
	}
}

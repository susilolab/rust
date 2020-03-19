fn main() {
	let a = [1,2,3];
	let iter = a.iter().map(|x| 2 * x);
	for x in iter {
		println!("{}", x);
	}
}

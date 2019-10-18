fn main() {
	let a = [1,2,3];
	let iter = a.into_iter().map(|x| 2 * x);
	for x in iter {
		println!("{}", x);
	}
}

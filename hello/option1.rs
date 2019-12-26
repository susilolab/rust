fn main() {
	let data = [Some(1), Some(2), Some(3)];
	let result: Vec<_> = data.iter().filter_map(|x| *x).collect();
	println!("{:?}", result);
}
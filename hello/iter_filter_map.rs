fn main() {
	let a = ["1", "lol", "3", "NaN", "5"];
	let iter: Vec<_> = a.iter()
		.map(|s| s.parse::<i32>())
		.filter_map(Result::ok)
		.collect();
	println!("{:?}", iter);
}

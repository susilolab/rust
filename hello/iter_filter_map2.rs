use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	let f = File::open("iter1.rs").unwrap();
	let f = BufReader::new(f);
	let v: Vec<u32> = f.lines()
		.filter_map(|line| line.ok().and_then(|l| l.parse().ok()))
		.collect();
	println!("{:?}", v);
}

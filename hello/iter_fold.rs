// `fold` mirip map reduce
fn main() {
	let nums = vec![1, 2, 3];
	println!("{}", nums.into_iter().fold(0, |acc, x| acc + x));
}

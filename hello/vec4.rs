/// Cetak vector menjadi string
/// e.g. [1,2,3,4,5]
///
fn main() {
	let mut val = vec![1,2,3,4,5];

	println!("{}", val.len());

	val.insert(1, 7);
	println!("[{}]", val.iter().fold(String::new(), |acc, &num| acc + &num.to_string() + ", "));
}
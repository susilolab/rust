use std::num::ParseIntError;

fn double_number(number_str: &str) -> Result<i32, ParseIntError> {
	match number_str.parse::<i32>() {
		Ok(n) => Ok(2 * n),
		Err(err) => Err(err),
	}
}
fn main() {
	let n = double_number("10").unwrap();
	println!("n: {}", n);

	let m = match double_number("5") {
		Ok(n) => n,
		Err(_) => return (),
	};
	println!("m: {}", m);
}
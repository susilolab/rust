fn main() {
	let x = get_prime(60);
	println!("{:?}", x);
}

fn get_prime(x: i32) -> Vec<i32> {
	let mut res = Vec::new();
	let mut divisor = 2;
	loop {
		let y = x/divisor;
		if y == 0 && {
			break;
		} else {
			divisor += 1;
		}
	}

	res
}

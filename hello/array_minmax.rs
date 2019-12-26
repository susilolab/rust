fn main() {
	let number = [10, 2, 8, 5, 9, 7, 6, 3, 1, 4];
	let mut min = number[0];
	let mut max = 0;

	for x in number.iter() {
		if x > &max {
			max = *x;
		}

		if x < &min {
			min = *x;
		}
	}
	println!("Min: {}, Max: {}", min, max);
}

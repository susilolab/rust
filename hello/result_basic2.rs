fn divide(x: f64, y: f64) -> Result<f64, ()> {
	if y == 0.0 {
		Ok(0.0)
	}else {
		Ok(x/y)
	}
}

fn main() {
	let x = match divide(3.0, 1.0) {
		Ok(val) => val,
		Err(_) => 0.0,
	};

	println!("nilai x: {}", x);
}
enum MatchError {
	DividedByZero,
}

fn divide(x: f64, y: f64) -> Result<f64, MatchError> {
	if y == 0.0 {
		Err(MatchError::DividedByZero)
	}else {
		Ok(x/y)
	}
}

fn main() {
	match divide(3.0, 0.0) {
		Ok(val) => println!("result: {}", val),
		Err(MatchError::DividedByZero) => println!("Division by zero"),
	}
}
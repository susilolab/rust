// Contoh trait lanjut
// Mendemokan implementasi trait `Add` dengan `RHS` default
// sehingga hanya menambahkan 2 tipe yang sama yaitu `Point`
//
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self {
			x: self.x + other.x,
			y: self.y + other.y,
		}
	}
}

impl Point {
	fn new(x: i32, y: i32) -> Self {
		Self {x, y}
	}
}
fn main() {
	let point1 = Point::new(1, 0);
	let point2 = Point::new(2, 3);

	println!("{:?}", &point1);
	println!("{:?}", &point2);
	
	let point3 = point1 + point2;
	println!("{:?}", point3);
}
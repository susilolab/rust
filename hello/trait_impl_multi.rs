pub struct Kotak {
	x: f64,
	y: f64,
}

// pub struct PersegiPanjang {
// 	x: f64,
// 	y: f64,
// }

trait HasArea {
	fn area(&self) -> f64;
}

trait HasCircumference {
	fn circumference(&self) -> f64;
}

// implementasi trait pada struct menggunakan keyword "for"
impl HasArea for Kotak {
    fn area(&self) -> f64 {
    	self.x * self.y
    }
}

impl HasCircumference for Kotak {
	fn circumference(&self) -> f64 {
		2.0 * (self.x + self.y)
	}
}

fn main() {
	let kotak = Kotak{x: 10.0, y: 20.0};
	println!("Luas: {:?}", kotak.area());
	println!("Keliling: {:?}", kotak.circumference());
}

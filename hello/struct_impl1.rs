pub struct Circle {
	x: f64,
	y: f64,
	radius: f64
}

// pada rust implementasi struct dapat dilakukan sebanyak mungkin
// seperti ini.
// lihat file struct_method1.rs
impl Circle {
	// taking self by ref
    fn area(&self) -> f64 {
    	std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Circle {
    // chaining method
    fn grow(&self, inc: f64) -> Circle {
    	Circle {x: self.x, y: self.y, radius: self.radius + inc}
    }
}

fn main() {
	let c: Circle = Circle{x: 2.0, y: 2.0, radius: 7.0};
	println!("{:?}", c.area());

	let d = c.grow(2.0).area();
	println!("{:?}", d);
}
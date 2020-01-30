pub struct Circle {
	x: f64,
	y: f64,
	radius: f64
}

// trait mirip interface pada golang
// trait merupakan daftar definisi fungsi tanpa body
trait HasArea {
	fn area(&self) -> f64;
}

// implementasi trait pada struct menggunakan keyword "for"
impl HasArea for Circle {
    fn area(&self) -> f64 {
    	std::f64::consts::PI * (self.radius * self.radius)
    }
}

fn main() {
	let circle = Circle { x: 2.0, y: 4.0, radius: 3.0 };
	println!("{}", circle.area());
}
pub struct Circle {
	x: f64,
	y: f64,
	radius: f64
}

impl Circle {
	// fungsi new dapat dipanggil seperti ini struct::nama_fungsi
	// dari pada struct.nama_fungsi
    fn new(x: f64, y: f64, radius: f64) -> Circle {
    	Circle {
    		x: x,
    		y: y,
    		radius: radius,
    	}
    }
}

fn main() {
	let c: Circle = Circle::new(2.0, 2.0, 7.0);
	println!("x: {}, y: {}, radius: {}", c.x, c.y, c.radius);
}
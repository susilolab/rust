// hal ini untuk menangani overloading pada rust,
// karena rust tidak support overloading. maka digunakanlah builder pattern
// yaitu dengan membuat struct mirip Circle dan menambahkan beberapa chain method.
pub struct Circle {
	x: f64,
	y: f64,
	radius: f64
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

pub struct CircleBuilder {
    opts: Circle,
}

impl CircleBuilder {
    fn new() -> Self {
        CircleBuilder { opts: Circle { x: 0.0, y: 0.0, radius: 1.0, } }
    }

    fn x(&mut self, coordinate: f64) -> &mut Self {
        self.opts.x = coordinate;
        self
    }

    fn y(&mut self, coordinate: f64) -> &mut Self {
        self.opts.y = coordinate;
        self
    }

    fn radius(&mut self, coordinate: f64) -> &mut Self {
        self.opts.radius = coordinate;
        self
    }

    fn finalize(&self) -> Circle {
        Circle { x: self.opts.x, y: self.opts.y, radius: self.opts.radius, }
    }
}

fn main() {
	let c = CircleBuilder::new()
        .x(1.0)
        .y(2.0)
        .radius(2.0)
        .finalize();
    println!("x: {}, y: {}, radius: {}", c.x, c.y, c.radius);
	println!("area: {}", c.area());
}

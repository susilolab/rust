use std::fmt;

#[derive(Debug)]
struct Triangle {
	a: f32,
	b: f32,
	c: f32
}

impl fmt::Display for Triangle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write(!f, "({}, {}, {})", self.a, self.b, self.c)
	}
}

fn main() {
	let triple = Triangle{a: 3.0, b: 4.0, c: 5.0 };
	println!("{}", triple);
}

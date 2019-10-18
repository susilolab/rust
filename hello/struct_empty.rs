struct Person;

impl Person {
	fn hello(&self) {
		println!("Hello brother");
	}
}

fn main() {
	let p: Person = Person{};
	p.hello();
}
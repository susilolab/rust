/// pretyy print menggunakan format `{:#?}`
///
#[derive(Debug)]
struct Person {
	name: &'static str,
	age: u8,
}

fn main() {
	let name = "Agus";
	let age = 35;
	let agus = Person { name, age };

	println!("{:#?}", agus);
}
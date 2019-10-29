fn main() {
	let hello = "Hello";
	println!("{}", hello);

	println!("Hello, world!");

	let a: i32 = 10;
	let b: i32 = 20;
	let c: i32;
	c = a+b;
	println!("{:?}", c);

	let benar: bool = true;
	println!("{}", benar);

	let angka = 10;
	println!("default type number: {}", angka);

	hello_fn();
}

fn hello_fn() {
	println!("Hello func");
}
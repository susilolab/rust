fn main() {
	println!("{}", inc(10));

	greeting(String::from("Agus"), 35);
}

fn inc(x: i32) -> i32 {
	return x+1;
}

fn greeting(name: String, age: i32) {
	println!("Halo bapak {}, umur Anda {}", name, age);
}
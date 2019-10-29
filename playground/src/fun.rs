pub fn fun_hello() {
	let name = "Agus";
	let demo = || -> String {
		let greeting = "Nama saya";
		format!("{} {}", greeting, name)
	};

	println!("Demo closure");
	let s = demo();
	println!("{}", s);

	let hello = |name: String| -> String {
		format!("{}", name)
	};

	println!("{}", hello(String::from("Hello world")));	
}
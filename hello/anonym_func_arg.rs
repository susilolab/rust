/// Contoh anonymous function pada rust
/// fungsi anonymous dimulai dengan tanda || dan expression atau || dan { body expression }
/// 
fn main() {
	// &str ke String
	// let str = "Hello".to_string();

	let nama = "Agus";
	let demo = || -> String {
		let greeting = "Nama saya: ";
		format!("{}{}", greeting, nama)
	};

	println!("Demo closure");
	let s = demo();
	println!("{}", s);

	let hello = |name: String| -> String {
		format!("{}", name)
	};

	println!("{}", hello(String::from("Hello world")));
}

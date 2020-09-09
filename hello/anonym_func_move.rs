/// Contoh `move` pada anonymous function
/// 
fn main() {
	let capture = "hello";
	let closure = move || {
		println!("rust says {}", capture);
	};

	closure();
	println!("{}", capture);

	let x = 5;
	std::thread::spawn(move || {
		println!("captured {} by value", x);
	}).join().unwrap();

	let y = x;
	cetak(y);
}

fn cetak(i: i32) {
	println!("{}", i);
}
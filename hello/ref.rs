/// Demo reference/ borrowing var
///  
fn main() {
	let name: String = "Agus".to_string();
	cetak(&name);
	cetak(&name);
	cetak(&name);

	println!("Nama saya: {}", name);
	let name_str = "Hello world!";
	cetak1(&name_str);
	cetak1(&name_str);
}

// fungsi cetak meminjam var name
fn cetak(name: &String) {
	println!("{}", name);
}

// fungsi cetak1 meminjam var name dengan type str
fn cetak1(name: &str) {
	println!("{}", name);
}

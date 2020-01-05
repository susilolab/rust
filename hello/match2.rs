fn main() {
	let x = 13;
	match x {
		1 => println!("satu"),
		2 | 3 | 5 | 7 | 11 => println!("bil prima"),
		13..=19 => println!("Anak muda"),
		_ => println!("Tidak ada yg spesial"),
	}

	let boolean = true;
	let binary = match boolean {
		false => 0,
		true => 1,
	};
	println!("{} -> {}", boolean, binary);
}
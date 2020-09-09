#![allow(dead_code)]
#![allow(unused_variables)]
use std::io;

#[derive(Debug)]
struct Kamus {
	id: i32,
	ina: String,
	eng: String, 
}

type Dict = Vec<Kamus>;

fn main() {
	// let mut dict: Dict = Vec::new();

	// dict.push(Kamus { id: 1, ina: "rumah".to_owned(), eng: "house".to_owned() });
	// println!("{:?}", dict);
	let mut choice = 0i32;
	let mut input = String::new();

	while choice != 4 {
		show_menu();
		io::stdin().read_line(&mut input);
		println!("input: {}", &input);
		if input.trim() != "" {
			choice = input.trim().parse::<i32>().unwrap();
		}
	}
}

fn show_menu() {
	let menus = r#"
\tApplikasi kamus\n\n
versi 1.0\n

1. Tambah Kamus
2. Lihat Kamus
3. Cari Kamus
4. Keluar
[1-3] Pilih menu:
"#;
	print!("{}", menus);
}

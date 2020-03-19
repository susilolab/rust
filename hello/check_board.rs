// 6,4
// ['O', 'X', 'O', 'X']
// ['X', 'O', 'X', 'O']
// ['O', 'X', 'O', 'X']

use std::io;
use std::io::Write;

struct CheckBoard {
	board: Vec<char>,
}

impl CheckBoard {
	fn new() -> Self {
		CheckBoard {
			board: Vec::new(),
		}
	}
}

fn main() {
	println!("Check Board");

	let row = prompt("Jumlah baris: ").parse::<i32>().unwrap();
	let col = prompt("Jumlah kolom: ").parse::<i32>().unwrap();

	let mut board: Vec<CheckBoard> = Vec::new();
	for x in 1..=row {
		for y in 1..=col {
			if x % 2 == 0 {
				if y % 2 == 0 {
					print!("O");
				} else {
					print!("X");
				}
			} else {
				if y % 2 == 0 {
					print!("X");
				} else {
					print!("O");
				}
			}
		}
		println!("");
	}
	println!("");

	println!("Baris: {}, Kolom: {}", row, col);
}

fn prompt(title: &str) -> String {
    print!("{}", title);
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    name.trim().to_string()
}
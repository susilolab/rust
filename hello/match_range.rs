fn main() {
	let x = 5;
    match x {
        1..=5 => println!("satu sampai lima"),
        _ => println!("yang lain"),
    }

    match_char();
}

fn match_char() {
	let x = 'c';
	match x {
		'a'..='j' => println!("awal huruf ASCII"),
		'k'..='z' => println!("akhir huruf ASCII"),
		_ => println!("yang lain"),
	}
}
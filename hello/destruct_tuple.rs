fn main() {
	let pair = (-2, 0);
	println!("Katakan padaku tentang {:?}", pair);

	match pair {
		(0, y) => println!("Pertama `0` dan `y` adalah `{:?}`", y),
		(x, 0) => println!("`x` adalah `{:?}` dan terakhir adalah `0`", x),
		_ => println!("Tidak masallah apa mereka itu"),
	}
}

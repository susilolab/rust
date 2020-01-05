/// rust punya 2 tipe data text yaitu `str` dan `String`
/// `str` untuk data statis/ tidak berubah ukurannya sedangkan `String` untuk data yang berkembang
///
fn main() {
	// &str to String
	let msg = "Hello ".to_string();
	// &str
	let nama = "Agus";
	let comment = msg+nama;
	println!("{}", comment);
}
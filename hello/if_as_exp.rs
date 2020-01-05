/// if sebagai expression
///
/// bisa dibaca pada contoh if berada didalam deklarasi variabel `txt`
/// body if tidak boleh ada `;` (semicolon) dan kurung kurawal terakhir harus
/// ada `;` (semicolonnya).
///
fn main() {
	let x = 5;

	let txt: String = if x == 5 {
		String::from("x is five")
	}else if x > 10 {
		String::from("x is big")
	}else {
		String::from("x is not five")
    };

    println!("{}", txt);
}
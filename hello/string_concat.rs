// belajar menggabung dua string yg berbeda tipe
// String dan &str
// untuk tipe 'str' menyusul

fn main() {
	// String dan &str
	let mut name: String = "Agus ".to_owned();
	let last_name: &str = "Susilo";

	name.push_str(last_name);
	println!("{}", name);

	// String dan String
	let mut lang_name: String = "Rust ".to_owned();
	let feature: String = "awesome".to_owned();
	// perhatikan pemakaian & pada saat push_str
	lang_name.push_str(&feature);
	println!("{}", lang_name);

	// String dan &str
	// no muttable var
	let lang: String = "Rust ".to_owned();
	let comment: &str = "is fast.";
	let lang_comment = lang+comment;
	println!("{}", lang_comment);

	// jika ingin menghasilkan string baru tanpa merubah keduanya
	let borrowed_str: &str = "Hello ";
	let an_borrowed_str: &str = "Rust!";
	let bersama = format!("{}{}", borrowed_str, an_borrowed_str);
	println!("{}", bersama);

	both_string();
	string_clone();
}

fn both_string() {
	let a: String = "Hello ".to_owned();
	let b: String = "world!".to_owned();
	let c = format!("{}{}", a, b);
	println!("{}", c);
}

fn string_clone() {
	let a: String = "Hello ".to_owned();
	let b: &str = "world clone.";
	let c = a.clone() + b;
	println!("{}", c);
}
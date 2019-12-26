/// variabel pada rust
/// 
/// rust tidak selalu mewajibkan mendeklarasikan tipe data var 
///
fn main() {
	let angka = 10;
	println!("Angka {}", angka);

	let list_lang = vec!["PHP", "Javascript", "Rust", "Elixir", "Go"];
	for x in list_lang {
		println!("{}", x);
	}
}
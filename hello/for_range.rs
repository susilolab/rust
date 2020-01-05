/// for in
///
/// digunakan untuk looping jangkauan misal `a..b`
/// `a..b` brarti dari a s/d sebelum b
/// kalau angka maka `1..10` dimulai dari 1 s/d 9
/// agar looping mengikutkan jangkauan terakhir maka gunakan
/// a..=b atau 1..=10
/// maka jangkauan terakhir akan dipakai
///
fn main() {
	println!("for i in 1..5");
	for i in 1..5 {
		print!(" {}", i);
	}
	println!("\n");

	println!("for i in 1..=5");
	for i in 1..=5 {
		print!(" {}", i);
	}
	println!("");
}

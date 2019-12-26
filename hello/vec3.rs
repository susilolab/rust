/// pada rust, vector tidak bisa di akses dua kali
/// untuk itu bisa menggunakan referensi
///
fn main() {
	let val = vec![1,2,3,4,5];

	for x in &val {
		println!("{:?}", x);
	}

	for y in &val {
		println!("{:?}", y);
	}
}

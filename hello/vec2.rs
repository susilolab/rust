/// pada rust, vector tidak bisa di akses dua kali
/// seperti dibawah ini
/// lihat vec3.rs
///
fn main() {
	let val = vec![1,2,3,4,5];

	for x in val {
		println!("{:?}", x);
	}

	// coba uncomment kode dibawah ini:
	// for y in &val {
	// 	println!("{:?}", y);
	// }
	// untuk sebabnya lihat file owner1.rs
}

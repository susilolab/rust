/// pada loop jika ingin mengembalikan suatu nilai dan menghentikan loop
/// bisa menggunakan break dengan value
///
fn main() {
	let mut counter = 0;
	let result = loop {
		counter += 1;

		if counter == 10 {
			break counter * 2;
		}
	};

	println!("result: {}", result);
}

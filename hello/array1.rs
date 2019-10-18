// iterator tidak di implementasi pada array
// maka digunakan .iter()
fn main() {
	let number = [100, 2, 3, 4];
	for i in number.iter() {
		println!("{:?}", i);
	}

	println!("jml elemen: {}", number.len());
	println!("elemen ke-1: {}", number[0]);
}
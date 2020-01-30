// konversi antar tipe dengan `as`
//
fn main() {
	let x: i32 = 10;
	let y: u32 = x as u32;

	println!("x: {}, y: {}", x, y);

	let i: i8 = 5;
	let j: i16 = i as i16;
	println!("i: {}, j: {}", i, j);
}

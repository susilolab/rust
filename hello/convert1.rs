// Ubah data dari tipe string ke integer
//
fn main() {
	let x = "10".to_string();
	let y: i32 = x.parse().unwrap();
	let z = x.parse::<i32>().unwrap();

	println!("y= {:?}", y);
	println!("z= {:?}", z);

	let a: i32 = 10;
	let b: i64 = a as i64;
	println!("b= {:?}", b);
}
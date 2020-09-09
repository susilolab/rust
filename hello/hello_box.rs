// Box
// adalah tipe pointer u/ mengalokasi heap(tumpukan)
// menyediakan bentuk yang paling sederhana dari pengalokasikan heap di rust.
// Box menyediakan ownership untuk alokasi ini, dan drop isinya 
// saat mereka keluar dari scope(lingkup)
//
use std::boxed::Box;

fn main() {
	let slot = Box::new(3);

	helper(&slot);
	helper(&slot);

	example();
}

fn helper(slot: &Box<i32>) {
	println!("The number was: {}", slot);
}

fn example() {
	let val: u8 = 5;
	let boxed: Box<u8> = Box::new(val);
	println!("{:?}", boxed);	
}
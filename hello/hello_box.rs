#![allow(warnings)]
#![feature(box_syntax, box_patterns)]

fn main() {
	let slot = box 3;

	helper(slot);
	helper(slot);
}

fn helper(slot: Box<int>) {
	println!("The number was: {}", slot);
}
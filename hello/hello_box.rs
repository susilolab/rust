#![allow(warnings)]
use std::boxed::Box;

fn main() {
	let slot = Box::new(3);

	helper(&slot);
	helper(&slot);
}

fn helper(slot: &Box<i32>) {
	println!("The number was: {}", slot);
}
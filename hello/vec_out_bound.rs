/*
Cara mengatasi out of bound vector
*/
fn main() {
	let val = vec![1,2,3,4,5];
	match val.get(7) {
		Some(x) => println!("Item 7 is {}", x),
		None => println!("Maaf vector ini terlalu pendek")
	}
}

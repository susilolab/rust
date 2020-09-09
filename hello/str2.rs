fn main() {
	let name = "agus";
	println!("{}", name.chars().nth(0).unwrap());

	println!("{:?}", &name[0..2]);
}
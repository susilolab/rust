fn main() {
	let mut foo: Option<i32> = Some(1i32);
	let i = foo.as_ref().unwrap();
	println!("{:?}", i);

	// let mut foo = Some(1i32);
	// let i = foo.as_ref().unwrap();
	// println!("{}", i);

	// foo = None;
	// println!("{:?}", i);
}

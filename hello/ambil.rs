const EXAMPLE: Option<i32> = Some(10);

fn main() {
	EXAMPLE.take();
	println!("{:?}", EXAMPLE);
	println!("{:?}", EXAMPLE);
	println!("{:?}", EXAMPLE);
}

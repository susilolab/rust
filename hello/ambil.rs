// Konstanta pada rust akan hidup pada seluruh program
// dan dapat digunakan berkali-kali dilingkup manapun
//
const EXAMPLE: Option<i32> = Some(10);

fn main() {
	EXAMPLE.take();
	println!("{:?}", EXAMPLE);
	println!("{:?}", EXAMPLE);
	println!("{:?}", EXAMPLE);
}

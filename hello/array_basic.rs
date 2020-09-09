// array static pada rust
// gunakan vec! untuk dinamik elemen
fn main() {
	let mut name = [""; 4];
	name[0] = "Agus";
	name[1] = "Susilo";
	name[2] = "Rust";
	name[3] = "Awesome";

	for n in name.iter() {
		println!("{}", n);
	}

	let mut lang: [i32; 3] = [0; 3];
	lang[0] = 1;
	lang[1] = 2;
	lang[2] = 3;

	for x in lang.iter() {
		println!("{:?}", x);
	}
}

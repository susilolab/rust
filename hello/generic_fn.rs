fn main() {
	let v = [1i, 2i, 3i];
	print_vec(v);

	let str_v = ["Hello", "you", "world"];
	print_vec(str_v);
}

fn print_vec<T: std::fmt::Display>(v: [T]) {
	for i in v.iter() {
		println!("{}", i);
	}
}
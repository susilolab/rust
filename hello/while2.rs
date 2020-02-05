// Menggunakan while untuk meng-iterasi data vector
//
fn main() {
	let num = vec![1,2,3,4,5];

	let mut iter = (&num).into_iter();
	while let Some(v) = iter.next() {
		println!("{}", v);
	}
}
//! contoh borrowin di rust
//! borrowing adalah meminjam sementara data dari pemiliknya, dengan tidak mengubah kepemilikan
// ! dari data tersebut
// !
// ! karena semua variabel di rust defaultnya owned
fn print_vec(data: &Vec<i32>) {
	for i in data {
		println!("{}", i);
	}
}

fn main() {
	let data = vec![1, 2, 3];
	print_vec(&data);
	print_vec(&data);
	print_vec(&data);
}
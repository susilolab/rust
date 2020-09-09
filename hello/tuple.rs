// Tuple adalah koleksi data dengan tipe yang berbeda
// tuple dibuat dengan syntax `()`
// jumlah elemen tak terbatas
// elemen tuple dapat diakses dengan index dimulai dari index 0.
//
// fungsi untuk merubah urutan data dengan tuple
#![allow(dead_code)]
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

// Struct model tuple
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
	let m = Matrix(1.0, 2.0, 3.0, 4.0);
	println!("{}", m.1);
}

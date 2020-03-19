#![allow(dead_code)]
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
	let m = Matrix(1.0, 2.0, 3.0, 4.0);
	println!("{}", m.1);
}


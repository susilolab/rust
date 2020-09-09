/// Data tuple dimulai dengan tanda ( dan )
/// cara akses tuple dengan menggunakan index
/// Struct juga dapat menjadi struk tuple
fn reverse(pair: (i32, bool)) -> (bool, i32) {
	let (integer, boolean) = pair;

	(boolean, integer)
}

fn main() {
	let x = (1, true);
	let rev = reverse(x);

	println!("t0 = {}, t1= {}", rev.0, rev.1);
}

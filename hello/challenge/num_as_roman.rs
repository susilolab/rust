use std::collections::HashMap;

fn main() {}

fn num_as_roman(num: i32) -> String {
	let mut kode = HashMap::new();
	kode.insert("I", 1);
	kode.insert("V", 5);
	kode.insert("X", 10);
	kode.insert("L", 50);
	kode.insert("C", 100);
	kode.insert("D", 500);
	kode.insert("M", 1_000);

	"Hello".to_owned()
}

#[test]
fn test_kembalian() {
	assert_eq!(num_as_roman(182), "CLXXXII");
	// C = 100
	// L = 50
	// X = 10
	assert_eq!(num_as_roman(1990), "MCMXC");
	// M = 1_000
	// C = 100
	// X = 10
	assert_eq!(num_as_roman(1875), "MDCCCLXXV");
}

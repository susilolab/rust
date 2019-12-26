fn main() {
	println!("{} days", 3);
	println!("{0}, this {1}. {1}, this is {0}", "Alice", "Bob");
	println!("{subject} {verb} {object}",
		object="t-shirt",
		subject="people",
		verb="using");
	println!("{} dari {:b} orang tahu angka biner, setengahnya tidak tahu", 1, 2);
	println!("{number:>width$}", number=1, width=6);
	println!("{number:>0width$}", number=1, width=6);
	println!("My name is {0}, {1} {0}", "Bond", "James");

	#[allow(dead_code)]
	struct Structure(i32);

	// println!("struk ini `{}` tidak akan dicetak...", Structure(3));
}
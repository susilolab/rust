fn main() {
	let names = vec!["Agus", "Susilo", "Nurhayati"];

	for (index, name) in names.iter().enumerate() {
		println!("[{}] {}", index, name);
	}
}

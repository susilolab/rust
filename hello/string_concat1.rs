fn main() {
	let fname = "Agus ";
	let lname = "Susilo";
	let full_name = format!("{}{}", fname, lname);
	println!("{}", full_name);

	println!("{}", hello());
}

fn hello() -> String {
	let fname = "Agus ";
	let lname = "Susilo";
	format!("{}{}", fname, lname)
}
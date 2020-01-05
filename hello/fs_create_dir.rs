use std::fs;
use std::env;
use std::path::Path;

fn main() {
	let tmp = match env::var("TEMP") {
		Ok(name) => name,
		Err(_e) => r#"C:\Users\susilo\AppData\Local\Temp"#.to_owned(),
	};

	let hello_rust = format!("{}\\hello_rust", tmp);
	let _ = fs::create_dir(&hello_rust);
	if Path::new(&hello_rust).exists() {
		println!("Folder exist");
	} else {
		println!("Folder tidak ada!");
	}
}
use std::process::Command;
use std::io::{self, Write};
use std::env;

// \.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin
fn main() {
	let rustc_path = match env::var("USERPROFILE") {
		Ok(val) => format!("{}\\.rustup\\toolchains\\stable-x86_64-pc-windows-msvc\\bin", val),
		Err(_) => String::from(""),
	};
	let file_name = match env::args().nth(1) {
		Some(val) => val,
		None => String::from(""),
	};

	if file_name.len() < 1 {
		println!("Nama file wajib ada!.");
		std::process::exit(1);
	}

	let output = Command::new("rustc.exe")
		.env("PATH", rustc_path)
		.arg(r#"d:\var\Rust\hello\bin"#)
		.arg("-v")
		.arg("--edition=2018")
		.arg("-C opt-level=s")
		.arg(file_name)
		.output()
		.expect("Gagal menjalankan perintah");
	io::stdout().write_all(&output.stdout).unwrap();
}

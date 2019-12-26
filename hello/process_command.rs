use std::process::Command;
use std::io::{self, Write};

fn main() {
	let output = Command::new("uutils.exe")
		.arg("ls")
		.arg("C:/Applications")
		.output()
		.expect("Gagal menjalankan perintah");
	// let hello = output.stdout;
	// println!("{:?}", hello);
	io::stdout().write_all(&output.stdout).unwrap();
}

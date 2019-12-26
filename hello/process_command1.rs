use std::process::Command;
use std::io::{self, Write};
// use std::env;

fn main() {
	let output = Command::new("php.exe")
		.env("PATH", r#"C:\Applications\phpstack\php74"#)
		.arg("-v")
		.output()
		.expect("Gagal menjalankan perintah");
	io::stdout().write_all(&output.stdout).unwrap();
}

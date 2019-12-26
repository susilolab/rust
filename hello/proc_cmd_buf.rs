use std::process::Command;
use std::io::{self, Write};



fn main() {
	compile("vec2.rs");
}

fn uutils() {
	let output = Command::new("uutils.exe")
		.arg("ls")
		.arg("C:/Applications")
		.output()
		.expect("Gagal menjalankan perintah");
	let res = output.stdout.as_slice();
	println!("{}", String::from_utf8_lossy(res));
	// io::stdout().write_all(&output.stdout).unwrap();
}

fn compile(file_name: &str) {
	let output = Command::new("rustc")
	    .args(&["--out-dir=bin", file_name])
	    .output()
	    .expect("Gagal menjalankan compiler");	
	let res = output.stdout.as_slice();
	println!("{}", String::from_utf8_lossy(res));
}

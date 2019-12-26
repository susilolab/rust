use std::process::Command;
// use std::io::{self, Write};

fn main() {
	let res = run_cmd();
    println!("{}", res);
}

fn run_cmd() -> String {
let output = Command::new("ls")
		.arg("/home/susilo")
		.output()
		.expect("Gagal menjalankan perintah");
	let res = output.stdout.as_slice();
	String::from_utf8_lossy(&res).to_string()
}

use std::process::{Command, Stdio};
use std::io::{self, Write};

fn main() {
	let child = Command::new("php.exe")
		.env("PATH", r#"C:\Applications\phpstack\php74"#)
		.arg("-v")
		.stdout(Stdio::piped())
		.spawn()
		.expect("Gagal menjalankan perintah");
	let output = child
		.wait_with_output()
		.expect("Gagal menunggu child");
	io::stdout().write_all(&output.stdout).unwrap();
}

use std::process::Command;
use std::io::{self, Write};
use std::ffi::OsStr;
use std::fs;

const CLI_APP: &str = r#"C:\Program Files\WinRAR\UnRaR.exe"#;

fn main() {
	for entry in fs::read_dir(r#"D:\Documents\eBooks\JavaScript"#).unwrap() {
		let dir = entry.unwrap();

		if dir.path().extension() == Some(OsStr::new("rar")) {
			let output = Command::new(CLI_APP)
				.args(&["x", "-c-", dir.path().to_str().unwrap()])
				.current_dir(r#"D:\Documents\eBooks\JavaScript"#)
				.output()
				.expect("Gagal menjalankan perintah");

			io::stdout().write_all(&output.stdout).unwrap();
		}
	}
}

use std::env;
use std::process::Command;
use std::fs;
use std::path::Path;
// use std::os::windows::prelude::*;
// use std::process;

fn main() {
	let prj_src = env::args()
		.nth(1)
		.unwrap_or_else(|| r#"/home/susilo/var/Rust"#.to_string());

	// let src: String = r#"d:\var\Rust"#.to_string();
    if let Ok(files) = get_files(prj_src) {
		for file in files {
			// let meta = fs::metadata(file.clone()).expect("error meta");
			// println!("{} -- {}", file, meta.file_size());
			let fname = format!("{}/Cargo.toml", file);
			if Path::new(&fname).exists() {
				let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
				// if file_name == "auto_task" {
					println!("{}", file);
					clean_prj(file);
				// }
			}
		}
	}
}

fn clean_prj(dirname: String) {
	let mut child = Command::new("cargo")
		.arg("clean")
		.arg("-v")
		.arg("--color")
		.arg("always")
		.current_dir(dirname)
		.spawn()
		.expect("failed to execute child");

	let ecode = child.wait().expect("failed to wait on child");
	assert!(ecode.success());
}

fn get_files(dirname: String) -> std::io::Result<Vec<String>> {
	let mut res: Vec<String> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path().display().to_string());
	}
	Ok(res)
}

use std::env;
// use std::process;
use std::fs;
// use std::os::windows::prelude::*;
use std::path::Path;

fn main() {
	let prj_src = env::args()
		.nth(1)
		.unwrap_or_else(|| r#"d:\var\Rust"#.to_string());

	// let src: String = r#"d:\var\Rust"#.to_string();
    if let Ok(files) = get_files(prj_src) {
		for file in files {
			// let meta = fs::metadata(file.clone()).expect("error meta");
			// println!("{} -- {}", file, meta.file_size());
			let fname = format!("{}\\Cargo.toml", file);
			if Path::new(&fname).exists() {
				println!("{}", file);
			}
		}
	}

	// let dirname: &'static str = "d:\\var";
	// println!("{}", dirname);
	// println!("{}", dirname);
	// println!("{}", dirname);
	// for entry in fs::read_dir(dirname)? {
		// let dir = entry?;
		// println!("{:?}", dir.path().display().to_string());
	// }
}

fn get_files(dirname: String) -> std::io::Result<Vec<String>> {
	let mut res: Vec<String> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path().display().to_string());
	}
	Ok(res)
}

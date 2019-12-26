/// du -h . --max-depth=0
///
// use std::process::Command;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    if let Ok(files) = get_files("/home/susilo/var/Rust") {
        for file in files {
            let path_name = format!("{}/Cargo.toml", &file);
            let path_exist = Path::new(&path_name).exists();
            // let base_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
            if path_exist {
                duh(&file);
                // println!("{}", base_name);
            }
        }
    }
}

fn duh(path_name: &str) {
	let output = Command::new("du")
		.arg("-h")
		.arg("--max-depth=0")
		.arg(path_name)
		.output()
		.expect("Gagal menjalankan perintah");
	io::stdout().write_all(&output.stdout).unwrap();
}

fn get_files(dirname: &str) -> std::io::Result<Vec<String>> {
	let mut res: Vec<String> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path().display().to_string());
	}
	Ok(res)
}

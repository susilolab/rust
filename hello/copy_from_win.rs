use std::env;
use std::fs;
use std::path::Path;

fn main() {
	let src = env::args()
		.nth(1)
		.expect("src harus diisi!.");
	let dst = env::args()
		.nth(2)
		.expect("dst harus diisi!.");

	let res = get_files(src).unwrap();
	// let dst_res = get_files(dst).unwrap();
	for name in res {
		let fname = Path::new(&name).file_name().unwrap().to_str().unwrap();
		let file_name = format!("{}/{}", dst, &fname);
		if !Path::new(&file_name).exists() {
			println!("{}", name);
		}
	}
}

struct FileInfo {
	name: String,
	file_name: String,
	path_name: String,
}

struct Files {
	names: Vec<FileInfo>,
}

impl Files {
	fn get_files(dirname: String) -> std::io::Result<Vec<FileInfo>> {
		let mut res: Vec<FileInfo> = Vec::new();

		for entry in fs::read_dir(dirname)? {
			let dir = entry?;
			res.push(dir.path().display().to_string());
		}
		Ok(res)
	}
}

use std::fs;
use std::path::Path;

fn main() {
	let src = "/mnt/drived/var/Rust/hello";
	let dst = "/home/susilo/var/Rust/hello";

	if !Path::new(&src).exists() {
		panic!("Lokasi sumber tidak ada!.");
	}

	let mut dst_files: Vec<String> = Vec::new();
	// tujuan
	for name in get_files(&dst.to_string()).unwrap() {
		let is_dir = name.path().is_dir();
		if is_dir {
			continue;
		}

		let file_name = name.path().file_name().unwrap().to_string_lossy().into_owned();
		let ext = name.path().extension().unwrap().to_string_lossy().into_owned();

		if ext == "rs" {
			dst_files.push(file_name.to_string());
		}
	}
	
	// sumber
	let mut cnt_files: i32 = 0;
	for name in get_files(&src.to_string()).unwrap() {
		let is_dir = name.path().is_dir();
		if is_dir {
			continue;
		}

		let file_name = name.path().file_name().unwrap().to_string_lossy().into_owned();
		let ext = match name.path().extension() {
			Some(val) => val.to_string_lossy().into_owned(),
			None => "".to_string(),
		};
		
		if ext == "rs" && !dst_files.contains(&file_name) {
			let _ = fs::copy(
				&name.path().display().to_string(),
				format!("{}/{}", dst, file_name)
			);
			cnt_files += 1;
			println!("Menyalin file {}..", file_name);
		}
	}

	if cnt_files == 0 {
		println!("Tidak ada file yang perlu disalin!.");
	}
}

fn get_files(dirname: &String) -> std::io::Result<Vec<fs::DirEntry>> {
	let mut res: Vec<fs::DirEntry> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir);
	}
	Ok(res)
}

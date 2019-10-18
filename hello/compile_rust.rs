use std::path::Path;
use std::fs;
use std::process::Command;

fn main() {
    run();
}

fn get_files(dirname: String) -> std::io::Result<Vec<String>> {
	let mut res: Vec<String> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path().display().to_string());
	}
	Ok(res)
}

fn run() {
    if let Ok(files) = get_files("D:/var/Rust/hello".to_string()) {
		for file in files {
			let paths = Path::new(&file);
			if !paths.is_file() {
				continue;
			}

			let fname = match paths.file_name() {
				Some(fname1) => 
					match fname1.to_str() {
						Some(name) => name,
						None => "",	
					},
				None => "",
			};

			let ext = match paths.extension() {
				Some(val) => match val.to_str() {
					Some(item) => item,
					None => "",
				},
				None => "",
			};

			let parent = match paths.parent() {
				Some(name) => match name.to_str() {
					Some(val) => val,
					None => "",
				},
				None => "",
			};

			if ext == "rs" {
				let file_name = format!("{}/{}", parent.to_owned(), &fname);
				println!("Mengkompile '{}'...", file_name);
                run_compiler(&file_name);
			}
		}
	}
}

fn run_compiler(file_name: &str) {
    Command::new("cmd")
        .args(&["/C", "rustc", "--out-dir=bin", file_name])
        .output()
        .expect("Gagal menjalankan compiler");
}

fn delete_debug_file() {

}
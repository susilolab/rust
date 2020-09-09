use std::fs;

const PHOTO_PATH: &str = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/20200123/DCIM/Camera";

fn main() {
    if let Ok(files) = get_files(PHOTO_PATH.to_string()) {
		for file in files {
			println!("{}", file);
		}
	}
}

fn get_files(dirname: String) -> std::io::Result<Vec<String>> {
	let mut res: Vec<String> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path().display().to_string());
	}
	Ok(res)
}
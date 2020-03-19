use std::fs;

const PHOTO1: &str = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/20191121-redmi_note_7/DCIM/Camera";
const PHOTO2: &str = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/_DCIM/Camera";
const PHOTO3: &str = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/DCIM/Camera";
const PHOTO4: &str = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/20200123/DCIM/Camera";
const PHOTO5: &str = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/20200229-0758/Camera";
const PHOTO6: &str = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/20200316-1013-DCIM/DCIM/Camera";

fn main() {
    if let Ok(files) = get_files(PHOTO1.to_string()) {
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
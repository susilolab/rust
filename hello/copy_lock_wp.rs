use std::env;
// use std::process;
use std::fs;
use std::os::windows::prelude::*;


fn main() {
	let src_path: String = r#"\AppData\Local\Packages\Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy\LocalState\Assets"#.to_string();
    let user_profile = match env::var("USERPROFILE") {
        Ok(val) => val, //format!("{}{}", val, SRC_PATH),
        Err(_e) => "".to_owned(),
    };

    let src = format!("{}{}", user_profile, src_path.clone());
    if let Ok(files) = get_files(src.clone()) {
		for file in files {
			let meta = fs::metadata(file.clone()).expect("error meta");
			println!("{} -- {}", file, meta.file_size());
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

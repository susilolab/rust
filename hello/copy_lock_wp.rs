use std::env;
use std::fs;
use std::os::windows::prelude::*;
use std::path::Path;

fn main() {
	let src_path: String = r#"\AppData\Local\Packages\Microsoft.Windows.ContentDeliveryManager_cw5n1h2txyewy\LocalState\Assets"#.to_string();
    let dst: String = r#"d:\var\tmp\wallpaper"#.to_owned();
    let user_profile = match env::var("USERPROFILE") {
        Ok(val) => val,
        Err(_e) => "".to_owned(),
    };

    if !Path::new(&dst).exists() {
    	let _ = fs::create_dir(&dst);
    }

    let src = format!("{}{}", user_profile, &src_path);
    if let Ok(files) = get_files(&src) {
		for file in files {
			let meta = fs::metadata(&file).expect("error meta");
			if meta.file_size() >= 200_000 {
				let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
				let dst_name = format!("{}\\{}.jpg", &dst, &file_name);
				
				println!("Menyalin file `{}`..", &file_name);
				let _ = fs::copy(&file, &dst_name);
			}
		}
	}    
}

fn get_files(dirname: &String) -> std::io::Result<Vec<String>> {
	let mut res: Vec<String> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path().display().to_string());
	}
	Ok(res)
}

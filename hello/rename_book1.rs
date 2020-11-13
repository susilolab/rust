use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Invalid argumen!.\nCoba jalankan seperti ini: rename_book /tmp");
        std::process::exit(1);
    }

    let folder_name = &args[1];
    if !folder_exists(folder_name.to_string()) {
        println!("Path folder tidak ada/Invalid!.");
        std::process::exit(1);
    }

    if let Ok(files) = get_files(folder_name.to_string()) {
        for file in files {
            let paths = Path::new(&file);
            if !paths.is_file() {
                continue;
            }

            let fname = match paths.file_name() {
                Some(fname1) => match fname1.to_str() {
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

            if ext == "rar" && fname.contains("Letmeread.net_") {
                let new_name = format!(
                    "{}/{}",
                    parent.to_owned(),
                    &fname.replace("Letmeread.net_", "")
                );
                let _ = fs::rename(&file, &new_name);
                println!("Merubah nama '{}' menjadi '{}'", file, new_name);
            }
            // println!("{}", paths.file_name().unwrap().to_str().unwrap());
            // if file.contains() {
            //fs::rename("/tmp/foo", "/tmp/bar").unwrap();
            // }
        }
    }
}

fn folder_exists(name: String) -> bool {
    if name.trim() == "" {
        return false;
    }

    let dir = Path::new(&name);
    dir.exists()
}

fn get_files(dirname: String) -> std::io::Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();

    for entry in fs::read_dir(dirname)? {
        let dir = entry?;
        res.push(dir.path().display().to_string());
    }
    Ok(res)
}

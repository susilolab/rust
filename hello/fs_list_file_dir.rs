use std::fs;

/// List file directory
/// 
/// Menampilkan daftar file didalam folder
/// 
/// @author  Agus Susilo
/// @created 22 September 2019 14:18 WIB
/// 
fn main() {
	if let Ok(files) = get_files("./".to_owned()) {
		for file in files {
			println!("{}", file);
		}
	}
}

/// Get files
/// Mengembalikan daftar file dengan tipe Array String
/// Kembalian harus di unwrap atau match
fn get_files(dirname: String) -> std::io::Result<Vec<String>> {
	let mut res: Vec<String> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path().display().to_string());
	}
	Ok(res)
}
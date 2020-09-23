use std::{
	fs,
	path,
	process::Command,
	env
};

fn main() {
	let folder = env::args().nth(1).expect("Nama folder harus di isi!.");
	if let Ok(folders) = get_files(&folder) {
		for x in folders {
			let p = format!("{}/target", x.display());
			if path::Path::new(&p).exists() {
				let mut child = Command::new("cargo")
					.args(&["clean", "-v", "--color", "always", "--offline"])
					.current_dir(x)
					.spawn()
					.expect("failed to execute child");
				let ecode = child.wait().expect("failed to wait on child");
				assert!(ecode.success());
			}
		}
	}
}

fn get_files(dirname: &str) -> std::io::Result<Vec<path::PathBuf>> {
	let mut res: Vec<path::PathBuf> = Vec::new();

	for entry in fs::read_dir(dirname)? {
		let dir = entry?;
		res.push(dir.path());
	}
	Ok(res)
}

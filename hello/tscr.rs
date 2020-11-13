use std::process::Command;
use std::env;

fn main() {
	match env::var("PATH") {
		Ok(val) => {
			let path = format!("{}:{}", val, "/home/susilo/.npm-global/bin");
			env::set_var("PATH", path);
		}
		Err(_) => (),
	}

	let file_name = env::args()
		.nth(1)
		.expect("Nama file tidak boleh kosong!.");

	let mut child = Command::new("tsc")
		.args(&["--outFile", file_name])
		.spawn()
		.expect("failed to execute child");

	let ecode = child.wait().expect("failed to wait on child");
	assert!(ecode.success());
}

use std::process::Command;

fn main() {
	let mut child = Command::new("cargo")
		.args(&["clean"])
		.current_dir("/home/susilo/var/Rust/os_ecc")
		.spawn()
		.expect("failed to execute child");
	let ecode = child.wait().expect("failed to wait on child");
	assert!(ecode.success());
}

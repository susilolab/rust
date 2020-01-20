use std::process::Command;

fn main() {
	let mut child = Command::new("ls")
		.current_dir("/home/susilo/var/Rust/blog")
		.spawn()
		.expect("failed to execute child");

	let ecode = child.wait().expect("failed to wait on child");
	assert!(ecode.success());
}
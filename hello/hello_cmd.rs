use std::process::Command;

fn main() {
	let mut child = Command::new("cmd.exe")
		.arg("/c")
		.arg("dir")
		.arg("d:")
		.spawn()
		.expect("failed to execute child");
	let ecode = child.wait().expect("failed to wait on child");
	assert!(ecode.success());
}
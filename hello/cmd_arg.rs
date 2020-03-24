use std::process::Command;

fn main() {
	let mut child = Command::new("ls")
		.arg("-lh")
		.arg("/home/susilo")
		.spawn()
		.expect("failed to execute child");

	let ecode = child.wait().expect("failed to wait on child");
	assert!(ecode.success());
}

use std::process::Command;

fn main() {
	let output = Command::new("sh")
					.arg("-c")
					.arg("echo hello")
					.output()
					.expect("failed to execute process");
	let hello = output.stdout;
	println!("{:?}", hello);
}

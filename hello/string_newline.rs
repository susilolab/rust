fn main() {
	let mut cmd = String::new();
	cmd.push_str("@echo off\n");
	cmd.push_str("coreutils.exe ls %*\n");

	println!("{}", cmd);
}
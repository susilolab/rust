use std::{
	io::prelude::*,
	process::{self, Command, Stdio},
	env
};

fn main() {
	let filename = env::args().nth(1).expect("Nama file harus di isi!.");
	let process = Command::new("7z")
		.args(&["l", "-ba", &filename])
		// .current_dir(x)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute child")
		.wait();

	match process.stdin.unwrap().write_all("1Qaz212".as_bytes()) {
		Ok(r) => println!("Set password 7z: {:?}", r),
		Err(why) => panic!("Tidak dapat menulis ke 7z stdin: {}", why),
	}

	// let _res = process.wait().unwrap();
	// println!("{:?}", res);
	// let mut s = String::new();
	// match process.stdout.unwrap().read_to_string(&mut s) {
 //    	Ok(_) => println!("{}", s),
 //    	Err(why) => panic!("Tidak dapat membaca 7z stdout: {}", why),
	// }
}

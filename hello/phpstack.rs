use std::thread;
use std::process::{Command, Stdio};
use std::io::{self, Write};

// #[derive(Debug)]
// enum CmdType {
// 	Nginx,
// 	Php,
// 	Redis,
// }

// struct Cmd {
// 	name: &str,
// 	env: &str,
// 	args: Vec<&str>,
// 	type: CmdType,
// }

fn main() {
	let t1 = thread::spawn(move || {
		start_php();
	});
	let t2 = thread::spawn(move || {
		start_nginx();
	});
	let t3 = thread::spawn(move || {
		start_redis();
	});
	t1.join().unwrap();
	t2.join().unwrap();
	t3.join().unwrap();
}

fn start_nginx() {
	println!("Memulai nginx..");
	let child = Command::new("nginx.exe")
		.env("PATH", r#"C:\Applications\phpstack\nginx"#)
		.arg("-c")
		.arg(r#"C:\Applications\phpstack\nginx\conf\nginx.conf"#)
		.arg("-p")
		.arg(r#"D:\var"#)
		.stdout(Stdio::piped())
		.spawn()
		.expect("Gagal menjalankan perintah nginx");

	let output = child
		.wait_with_output()
		.expect("Gagal menunggu child");
	io::stdout().write_all(&output.stdout).unwrap();
}

fn start_php() {
	println!("Memulai php..");
	let child = Command::new("php-cgi.exe")
		.env("PATH", r#"C:\Applications\phpstack\php74"#)
		.arg("-b")
		.arg("127.0.0.1:9000")
		.stdout(Stdio::piped())
		.spawn()
		.expect("Gagal menjalankan perintah php");

	let output = child
		.wait_with_output()
		.expect("Gagal menunggu child");

	io::stdout().write_all(&output.stdout).unwrap();
}

fn start_redis() {
	println!("Memulai redis..");
	let child = Command::new("redis-server.exe")
		.env("PATH", r#"C:\Applications\redis"#)
		.stdout(Stdio::piped())
		.spawn()
		.expect("Gagal menjalankan perintah redis");

	let output = child
		.wait_with_output()
		.expect("Gagal menunggu child");

	io::stdout().write_all(&output.stdout).unwrap();
}


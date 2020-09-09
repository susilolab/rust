// Aplikasi kecil ini berguna untuk memperbarui command dari coreutils.exe yang semula bernama uutils.exe
//
// Agus Susilo
// 10 Juni 2020 22:24
// 
#![allow(dead_code)]
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{self, BufRead};


fn main() {
	let cmds = vec![
		"arch", "base32", "base64", "basename", "cat", "cksum", "comm", "cp", "cut", "date", "df", "dircolors", "dirname",
		"echo", "env", "expand", "expr", "factor", "false", "fmt", "fold", "hashsum", "head", "hostname", "join", "link", "ln",
		"ls", "md5sum", "mkdir", "mktemp", "more", "mv", "nl", "nproc", "od", "paste", "printenv", "printf", "ptx", "pwd",
		"readlink", "realpath", "relpath", "rm", "rmdir", "seq", "sha1sum", "sha224sum", "sha256sum", "sha3-224sum",
		"sha3-256sum", "sha3-384sum", "sha3-512sum", "sha384sum", "sha3sum", "sha512sum", "shake128sum",
		"shake256sum", "shred", "shuf", "sleep", "sort", "split", "sum", "sync", "tac", "tail", "tee", "test", "touch", "tr",
		"true", "truncate", "tsort", "unexpand", "uniq", "wc", "whoami", "yes"
	];

	for name in cmds {
		let file_name = format!("c:/Applications/bin/{}.cmd", name);
		if !Path::new(&file_name).exists() {
			continue;
		}

		if let Ok(lines) = read_lines(&file_name) {
			let mut contents = String::new();

			for line in lines {
				if let Ok(ip) = line {
					if ip.starts_with("uutils.exe") {
						contents.push_str(&ip.replace("uutils.exe", "coreutils.exe").to_owned());
						contents.push_str("\n");
					} else {
						contents.push_str(&ip.to_owned());
						contents.push_str("\n");
					}
				}
			}
			update_cmd(&file_name, &contents);
			println!("Memperbarui perintah `{}`..", name);
		}
	}
}

fn update_cmd(file_name: &str, contents: &str) {
	let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_name);

	match file {
        Err(e) => eprintln!("error open file: {}", e),
        Ok(mut f) => {
            let _ = f.write_all(contents.to_owned().as_bytes());
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

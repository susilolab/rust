use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
	if let Ok(lines) = read_lines(r#"C:\Users\susilo\Downloads\app.log"#) {
		let mut x: i32 = 0;
		for line in lines {
			if let Ok(ip) = line {
				x = x + 1;
				println!("{}\t{}", x, ip);
			}
		}
	}
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}
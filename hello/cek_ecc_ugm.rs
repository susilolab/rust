use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
	if let Ok(mut stream) = TcpStream::connect("ecc.co.id:80") {
		let mut res = String::new();
		// stream.read(&mut [0; 128]).unwrap();
		stream.read_to_string(&mut res).unwrap();
		println!("{:?}", res);
	} else {
		println!("Tidak dapat terhubung ke ecc.co.id!.");
	}
}

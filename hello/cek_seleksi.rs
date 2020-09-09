use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
	let addrs = vec![
		"10.1.8.91",
		"10.1.8.92",
		"10.1.80.58",
		"10.1.80.59",
		"10.1.8.93",
		"10.1.8.95",
	];

	for addr in addrs {
		if let Ok(mut stream) = TcpStream::connect(addr) {
			let mut res = String::new();
			// stream.read(&mut [0; 128]).unwrap();
			stream.read_to_string(&mut res).unwrap();
			println!("{:?}", res);
	
		} else {
			println!("Tidak dapat terhubung ke ip {}.", addr);
		}
	}
}

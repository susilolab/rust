use std::net::TcpStream;

fn main() {
    for i in 1..1025 {
        let addr = format!("ecc.co.id:{}", i);
        if let Ok(_stream) = TcpStream::connect(addr) {
            println!("{} open", i);
        } else {
            continue;
        }
    }
}

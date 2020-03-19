use std::net::{SocketAddr, TcpStream};

fn main() {
    if let Ok(_stream) = TcpStream::connect("ecc.co.id:80") {
        println!("Ok");
    } else {
        println!("Error");
    }
}

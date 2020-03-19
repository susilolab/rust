use std::net::TcpStream;
use std::thread;

fn main() {
    let mut threads = Vec::new();

    for i in 1..1025 {
        let t = thread::spawn(move || {
            let addr = format!("ecc.co.id:{}", i);
            if let Ok(_stream) = TcpStream::connect(addr) {
                println!("{} open", i);
            }
        });
        threads.push(t);
    }

    for t in threads {
        t.join().expect("thread failed");
    }
}

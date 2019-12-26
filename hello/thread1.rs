use std::thread;

fn main() {
    let child = thread::spawn(move || {
        println!("Hello thread");
    });
    let res = child.join();
}
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::channel;
use std::sync::mpsc;
use std::thread;

fn main() {
	let (tx, rx) = channel();

	for i in 0..10 {
		let tx = tx.clone();
		thread::spawn(move || {
			tx.send(i).unwrap();
		});
	}

	for i in 0..10 {
		println!("{}", rx.recv().unwrap());
	}
}

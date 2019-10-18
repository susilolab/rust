use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

fn main() {
	let (chan, port) = channel();

	spawn(proc() {
		chan.send(10u);
	});
	println!("{:s}", port.recv().to_str());
}
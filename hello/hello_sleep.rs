use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {
	let now = Instant::now();
	loop {
		sleep(Duration::new(1, 0));
		println!("{}", now.elapsed().as_secs());
	}
}
use std::collections::HashMap;
use std::thread;
use std::time::Instant;

const NUM_ELEMENTS: usize = 1_000_000;
type HeayThings = HashMap<usize, Vec<usize>>;

fn main() {
	let t1 = make_heavy_things();
	let t2 = make_heavy_things();

	let len = log_time("drop di thread lain", || {
		drop_in_other_thread(t2)
	});
	assert_eq!(len, NUM_ELEMENTS);

	let len = log_time("drop di thread ini", || {
		drop_things(t1)
	});
	assert_eq!(len, NUM_ELEMENTS);
}

fn make_heavy_things() -> HeayThings {
	(1..=NUM_ELEMENTS).map(|v| (v, vec![v])).collect()
}

fn drop_things(things: HeayThings) -> usize {
	things.len()
}

fn drop_in_other_thread(things: HeayThings) -> usize {
	let len = things.len();
	thread::spawn(move || drop(things));
	len
}

fn log_time<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
	let time = Instant::now();
	let result = f();
	println!("{} {:?}", name, time.elapsed());
	result
}
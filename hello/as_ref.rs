fn main() {
	let vec = Vec::new();
	work_on_bytes(&vec);

	let arr = [0; 10];
	work_on_bytes(&arr);

	let slice = &[1, 2, 3];
	work_on_bytes(slice);
}

fn work_on_bytes(slice: &[u8]) {
}
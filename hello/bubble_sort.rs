fn main() {
	let mut data = [1,5,2,4,8];
	bubble_sort(&mut data);
	println!("{:?}", data);
}

fn bubble_sort(arr: &mut [i32]) {
	let mut swapped = true;
	while swapped {
		swapped = false;
		for i in 1..arr.len() {
			if arr[i - 1] > arr[i] {
				arr.swap(i - 1, i);
				swapped = true;
			}
		}
	}

}

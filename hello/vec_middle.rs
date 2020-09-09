fn get_middle(stack: &[i32]) -> Option<i32> {
	let size = stack.into_iter().count();
	println!("{}", size);
	None
}

fn main() {
	let num = vec![1,2,3,4];
	get_middle(&num);
}

fn main() {
	let mut stack: Vec<i32> = Vec::new();
	push(&mut stack, 1);
	push(&mut stack, 2);
	for i in stack {
		println!("{}", i);
	}
}

fn pop(stack: &mut Vec<i32>) -> Option<i32> {
	if stack.len() > 0 {
		let val = stack.pop().unwrap();
		Some(val);
	}
	None
}

fn push(stack: &mut Vec<i32>, x: i32) {
	if stack.len() < 3 {
		stack.push(x);
	}
}
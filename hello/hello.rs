fn main() {
	let (x,y) = (10,20);

	println!("x: {}, y: {}", x, y);
	let mut z = 5;
	println!("{}", z);
	z= 15;
	hello_fn();

	let v = vec![1,2,3];
	println!("{}", v[0]);

	println!("inc: {}", inc(10));
}

fn hello_fn() {
	println!("Hello world");
}

fn inc(x: i32) -> i32 {
	return x+1;
}
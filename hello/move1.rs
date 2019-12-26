fn main() {
	let hello = "Hello World";
	{
	let say = || {
		println!("say {}", hello);
	};

	say();
	}
}
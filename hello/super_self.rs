/// `super` untuk mengakses diluar module saat ini
/// `self` untuk mengakses current module
/// `crate` untuk root module 
fn function() {
	println!("Called `function()`");
}

mod cool {
	pub fn function() {
		println!("Called `cool::function()`");
	}
}

mod my {
	pub fn function() {
		println!("Called `my::function()`");
	}

	mod cool {
		pub fn function() {
			println!("Called `my::cool::function()`");
		}
	}

	pub fn indirect_call() {
		println!("Called `my::indirect_call()`, that\n> ");

		self::function(); // mengakses `my::function()`
		function(); // sama seperti diatas

		self::cool::function();

		super::function(); // mengakses diluar `my`

		{
			use crate::cool::function as root_function;
			root_function();
		}
	}
}

fn main() {
	my::indirect_call();
}
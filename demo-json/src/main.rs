extern crate rustc_serialize;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Todo {
	id: i32,
	title: String,
}

fn main() {
	let obj = Todo {
		id: 1,
		title: "Create web".to_string(),
	};

	let enc = json::encode(&obj).unwrap();
	println!("{:?}", enc);

	let dec: Todo = json::decode(&enc).unwrap();
	println!("{}", dec.title);
}
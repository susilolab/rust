// Return value and scopes
//
fn main() {
	let s1 = gives_ownership();
	let s2 = String::from("Hello");
	let s3 = take_and_gives_back(s2);
}

fn gives_ownership() -> String {
	let some_string = String::from("hello");
	some_string
}

fn take_and_gives_back(a_string: String) -> String {
	a_string
}
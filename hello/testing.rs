#[use_macro]
#[test]
fn this_tests_code() {
	println!("");
}
// compile with rustc --test name.rs

#[test]
fn second_tests() {
	fail!("Fail");
}
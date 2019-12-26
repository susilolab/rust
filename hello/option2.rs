fn main() {
	let msg = Some("Hello world");
	if let Some(ref m) = msg {
		println!("{}", *m);
	}
}
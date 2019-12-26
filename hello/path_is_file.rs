use std::path::Path;

fn main() {
	let path = r#"D:\var\Rust\hello\path_ext.rs"#.to_string();
	assert_eq!(Path::new(&path).is_file(), true);
}

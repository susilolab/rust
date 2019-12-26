use std::env;

fn main() {
	let config_file = env::args()
		.nth(1)
		.unwrap_or_else(|| r#"C:\Windows\Temp"#.to_string());

	println!("Config file path: {}", config_file);
}
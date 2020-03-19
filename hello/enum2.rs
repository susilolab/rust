#[allow(dead_code)]
enum Color {
	Red,
	Blue,
	Green,
	RGB(u32, u32, u32),
	HSV(u32, u32, u32),
	HSL(u32, u32, u32),
	CMY(u32, u32, u32),
	CMYK(u32, u32, u32, u32),
}

fn main() {
	let color = Color::RGB(122, 17, 40);
	println!("Warna apa ini?");

	match color {
		Color::Red => println!("Warnanya merah"),
		Color::Blue => println!("Warnanya biru"),
		Color::Green => println!("Warnanya hijau"),
		Color::RGB(r, g, b) => 
			println!("Merah: {}, hijau: {}, biru: {}!", r, g, b),
		Color::HSV(h, s, v) =>
			println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
		Color::HSL(h, s, l) =>
			println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
		Color::CMY(c, m, y) =>
			println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
		Color::CMYK(c, m, y, k) =>
			println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!", c, m, y, k),
	}
}
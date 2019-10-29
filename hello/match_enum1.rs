enum Color {
    Hijau,
    Biru,
}

fn main() {
    let warna: Color = Color::Biru;
    match warna {
        Color::Biru => println!("Biru"),
        Color::Hijau => println!("Hijau"),
    }
}
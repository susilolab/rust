/// Ownership/ Kepemilikan
///
/// Ahad, 15 Desember 2019 18:00
/// Demo untuk membuktikan bahwa fungsi println! tidak mentransfer
/// kepemilikan
///
fn main() {
    // buat var name dengan default immutable/ tidak dapat diubah
    let name = String::from("Hello");
    println!("1. {}", name);
    println!("2. {}", name);
    println!("3. {}", name);

    let msg1 = format!("Pesan 1: {}", name);
    let msg2 = format!("Pesan 2: {}", name);
    println!("{}", msg1);
    println!("{}", msg2);
}

// contoh dinamik array pd rust
// dinamik array pd rust disebut vector
fn main() {
    let langs = vec!["Rust", "Go", "PHP", "Nodejs"];
    // pada vector tidak perlu menggunakan fungsi .iter saat loop data
    for val in langs {
        println!("{}", val);
    }
}
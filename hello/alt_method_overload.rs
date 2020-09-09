/// Contoh implementasi alternative overloading method di rust
/// yaitu dengan menggunakan generik trait
///
// buat struk kosongan 
struct Detector;

// buat generic trait
trait Detect<T> {
    fn detect(input: T);
}

// implementasi trait untuk tipe integer 32
impl Detect<i32> for Detector {
    fn detect(input: i32) {
        println!("{} is i32", input);
    }
}

// implementasi trait untuk tipe string
impl Detect<&'static str> for Detector {
    fn detect(input: &'static str) {
        println!("{} is &str", input);
    }
}

fn main() {
    Detector::detect(10);
    Detector::detect("Hello");
}

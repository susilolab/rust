/// Contoh implementasi alternative overloading method di rust
/// yaitu dengan menggunakan generik trait
/// 
struct Detector;

trait Detect<T> {
    fn detect(input: T);
}

impl Detect<i32> for Detector {
    fn detect(input: i32) {
        println!("{} is i32", input);
    }
}

impl Detect<&'static str> for Detector {
    fn detect(input: &'static str) {
        println!("{} is &str", input);
    }
}

fn main() {
    Detector::detect(10);
    Detector::detect("Hello");
}

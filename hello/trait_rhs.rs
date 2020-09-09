// http://localhost/rust/rust-book/ch19-03-advanced-traits.html
//
// Contoh implementasi trait dengan mengisi RHS dengan nilai lain
// RHS = Right Hand Side
//
use std::ops::Add;

#[derive(Debug)]
struct Milimeters(u32);
struct Meters(u32);

// trait Add<RHS=Self>
// RHS kita isi dengan struct `Meters` sehingga kita bisa menambah dengan struct lain
impl Add<Meters> for Milimeters {
    type Output = Milimeters;

    fn add(self, other: Meters) -> Milimeters {
        Milimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let mm = Milimeters(10);
    let m = Meters(2);
    let total = mm + m;
    println!("{:?}", total);
}

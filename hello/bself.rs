trait T {
    type Item;
    const C: i32;

    // `Self` akan menjadi tipe apapun yang mengimplementasi `T`.
    fn new() -> Self;
    // `Self::Item` akan menjadi alias tipe pada implementasi
    fn f(&self) -> Self::Item;
}

#[derive(Debug)]
struct S;

impl T for S {
    type Item = i32;
    const C: i32 = 9;
    // `Self` adalah tipe `S`.
    fn new() -> Self {
        S
    }

    // `Self::Item` adalah tipe i32.
    fn f(&self) -> Self::Item {
        Self::C // `Self::C`
    }
}

fn main() {
    let s: S = S {};

    println!("{:?}", s.f());
}

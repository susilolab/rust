// multiple trait require yg harus di implementasi
#![allow(dead_code)]
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
struct A(i32);

// implementasi trait Display dan Clone agar dapat ditaruh
// difungsi req_both
impl Display for A {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}

fn req_both<T: Display + Clone>(a: T) {
    let b = a.clone();
    println!("{}", b);
}

fn main() {
    let a = A(10);
    req_both(a);
}
use std::mem;

fn main() {
    let a: bool;
    let a = a == true;
    println!("{}", mem::size_of_val(&a));
}

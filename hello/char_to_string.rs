use std::iter::FromIterator;

fn main() {
    let v = vec!['a', 'b', 'c', 'd'];
    let s = String::from_iter(v);

    println!("{}", s);
}

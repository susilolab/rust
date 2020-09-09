// `last` menerima input iter dan menghasilkan `Option<T>` elemen terakhir
fn main() {
    let a = [1, 2, 3];
    assert_eq!(a.iter().last(), Some(&3));
    // output `a.iter().last()`: Some(&3)
}

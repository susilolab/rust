fn main() {
    let a = [1, 2, 3];
    assert!(a.iter().any(|x| x % 2 == 0 || x % 3 == 0));
}
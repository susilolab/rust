// `any` memeriksa setiap elemen apakah cocok dengan kondisi
// dalam contoh ini kondisinya adalah
// `x % 2 == 0 || x % 3 == 0`
//
fn main() {
    let a = [1, 2, 3];
    assert!(a.iter().any(|x| x % 2 == 0 || x % 3 == 0));
}

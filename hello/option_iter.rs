fn main() {
    let x = Some("string");
    let v: Vec<&str> = x.into_iter().collect();
    assert_eq!(v, ["string"]);
    println!("{:?}", v);
}
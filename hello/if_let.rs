fn main() {
    let s = Some("a");

    if let Some(x) = s {
        println!("x: {}", x);
    } else {
        println!("None");
    }
}

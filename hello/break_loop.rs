// rust mendukung break loop dengan mengembalikan nilai
//
fn main() {
    let x = loop {
        break 7;
    };
    println!("{}", x);
}

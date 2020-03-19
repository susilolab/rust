// pengunaan `for` secara terbalik
// biasanya dari x ke y
// kalau dari y ke x pake `range.rev()`
//
// 23 Januari 2019
//
fn main() {
    println!("Hello");
    for x in (6..=8).rev() {
        println!("x: {}", x);
    }
}

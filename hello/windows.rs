fn main() {
    let slice = ['a', 'b', 'c', 'd'];
    let mut iter = slice.windows(2);
    for x in iter {
        println!("{:?}", x);
    }
    // tergantung argumen `count` pada fungsi `windows`
    // ['a', 'b']
    // ['c', 'd']
}

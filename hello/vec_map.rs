fn main() {
    for x in vec![1, 2, 3].into_iter().map(|y| y + 1) {
        println!("{}", x);
    }
}

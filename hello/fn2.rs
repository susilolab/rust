fn main() {
    let double = |x| x * 2;
    println!("{:?}", double_with_two(double));
}

fn double_with_two<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(2)
}

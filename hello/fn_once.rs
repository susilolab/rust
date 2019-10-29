fn main() {
    let double = |x| x * 2;
    println!("{:?}", double_with_two(double));

    let doble = |x| x * 4;
    println!("{:?}", double_once(doble));
    println!("{:?}", double_once(doble));
}

fn double_with_two<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(2)
}

fn double_once<F>(f: F) -> i32
where
    F: FnOnce(i32) -> i32,
{
    f(2)
}

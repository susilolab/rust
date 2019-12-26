fn main() {
    let x = 5;
    let _y = double(x);
    println!("{}", x);
}

fn double(x: i32) -> i32 {
    x * 2
}
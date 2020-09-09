fn main() {
    let a = -121;
    println!("{:?}", is_palindrom(a));
}

fn is_palindrom(x: i32) -> bool {
    x.to_string() == x.to_string().chars().rev().collect::<String>()
}

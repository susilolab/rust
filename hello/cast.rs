fn cast(x: f32) -> u8 {
    x as u8
}

fn main() {
    let f = 300.0;
    let x = cast(f);
    println!("x: {}", x);
}

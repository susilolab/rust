fn main() {
    let x: i32 = 10;
    let y: u32 = to_u32(x);
    println!("{}", y);
}

fn to_u32(v: i32) -> u32 {
    //u32::from(v)
    v.into()
}

fn main() {
    let a = "a".parse::<i32>();
    match a {
        Ok(val) => println!("{:?}", val),
        Err(e) => println!("{:#?}", e),
    }
    //println!("{:?}", a);
}

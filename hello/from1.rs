use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    assert_eq!(u16::from(13u8), 13u16);

    let num = Number::from(30);
    println!("My number is {:?}", num);
}

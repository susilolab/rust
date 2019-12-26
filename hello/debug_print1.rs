#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!(
        "{:#?}",
        Person {
            name: "Agus",
            age: 35
        }
    );
}

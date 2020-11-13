use std::io;
use std::io::Write;

fn main() {
    print!("Ketikan namamu: ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    println!("Namamu: {}", name);
}


use std::env;

fn main() {
    match env::var("USERPROFILE") {
        Ok(val) => println!("{}", val),
        Err(e) => println!("{}", e),
    }
}

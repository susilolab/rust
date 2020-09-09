use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("/home/susilo/var/Rust/ralert/ExperienceLevels.xml").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    
    println!("{:?}", s);
}
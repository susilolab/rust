use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i32,
    title: String,
}

fn main() -> std::io::Result<()> {
    let f = File::open("todo.json")?;
    let mut buf_reader = BufReader::new(f);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let res: Todo = serde_json::from_str(&contents)?;
    println!("{:?}", res);

    Ok(())
}

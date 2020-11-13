use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: i32,
    title: String,
}

fn main() {
    let obj = Todo {
        id: 1,
        title: "Create web".to_string(),
    };

    let enc = serde_json::to_string(&obj).unwrap();
    println!("{}", enc);

    let dec: Todo = serde_json::from_str(&enc).unwrap();
    println!("{:?}", dec);
}

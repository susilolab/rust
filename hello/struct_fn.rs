struct User {
    id: i32,
}

impl User {
    pub fn table_name() -> String {
        "user".to_owned()
    }
}

fn main() {
    println!("{}", User::table_name());
}

fn foo(words: &[&str]) {
    match words {
        ["Foo", "Bar", ..] => println!("Baz"),
        [.., "!"] => println!("!!!"),
        [start @ .., "z"] => println!("mulai dengan {:?}", start),
        ["Foo", end @ ..] => println!("akhir dengan {:?}", end),
        rest => println!("{:?}", rest),
    }
}

fn main() {
    foo(&["Foo", "Bar", "z"]); // Baz
    foo(&["Foo", "Baz", "z"]); // mulai dengan ["Foo", "Baz"]
    foo(&["Baz", "z", "!"]); // !!!
    foo(&["Foo", "Hello", "World"]); // akhir dengan ["Hello", "World"]
}

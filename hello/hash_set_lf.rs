use std::collections::HashSet;

fn main() {
    let ho = hello_one("A");
    println!("{}", ho);

    let a = &["a", "b"];
    let b = hello(a);
    println!("{:?}", b);
}

fn hello_one<'a>(s: &'a str) -> &'a str {
    s
}

fn hello<'a>(data: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for x in data.iter() {
        hs.insert(*x);
    }

    hs
}

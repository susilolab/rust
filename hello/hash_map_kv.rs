use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');

    println!("{:?}", map.get(&'[') == Some(&']'));

    let mut stack = Vec::new();
    stack.push(']');

    let x = '[';
    match (stack.pop().as_ref(), map.get(&x)) {
        (Some(a), Some(b)) => println!("a, b: {:?}", a == b),
        (Some(_a), None) => println!("a"),
        (None, Some(_b)) => println!("b"),
        (None, None) => println!("none"),
    }
    // println!("{:?}", map.get(&'[') == stack.pop().as_ref());
}

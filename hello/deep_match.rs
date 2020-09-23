use std::collections::HashMap;

fn main() {
    let x = ')';
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');

    let mut stack: Vec<char> = Vec::new();
    stack.push('{');

    let mut res = true;
    res = match stack.pop() {
        Some(last) => match map.get(&last) {
            Some(&a) => x == a,
            None => false,
        },
        None => false,
    };

    println!("res= {:?}", res);
}

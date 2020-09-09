use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert('(', ')');

    println!("{:?}", map.get(&'(') == Some(&')'));

    cocok("{}");
}

fn cocok(string: &str) {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');

    let mut stack = Vec::new();
    let mut res = true;

    for x in string.chars() {
        if x == '(' || x == '{' || x == '[' {
            stack.push(x);
        } else {
            let last = match stack.pop() {
                Some(val) => val,
                None => '-'
            };

            println!("last: {:?}", last);
            if last == '-' {
                res = false;
            } else {
                res = true
            }

            let hash = if let Some(val) = map.get(&last) {
                val
            } else {
                &'-'
            };

            println!("hash: {:?}", hash);
            if hash == &'-' {
                res = false;
            }

            if hash != &last {
                res = false;
            }

            println!("x: {:?}, stack: {:?}", x, stack.pop());
        }
    }

    if stack.len() != 0 {
        res = false;
    }
    
    println!("res: {:?}", res);
}
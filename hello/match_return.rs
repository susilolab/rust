use std::collections::HashMap;

fn main() {
    let s = "{)";
    println!("========== Balanced One ==========");
    balanced_one(s);

    let s = "{)";
    println!("========== Balanced ==========");
    println!("balanced: {:?}", balanced(s));
}

fn balanced_one(string: &str) {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');

    let mut stack = Vec::new();

    for x in string.chars() {
        match x {
            '(' | '[' | '{' => stack.push(x),
            ')' | ']' | '}' => {
                match stack.pop() {
                    Some(last) => {
                        match map.get(&last) {
                            Some(&b) => println!("x= {}, b= {}, x == b: {:?}", x, b, x == b),
                            None => println!("map none")
                        }
                    }
                    None => println!("stack none")
                }
            }
            _ => println!("lainnya")
        }
    }
}

fn balanced(string: &str) -> bool {
    let mut map = HashMap::new();
    map.insert('(', ')');
    map.insert('[', ']');
    map.insert('{', '}');

    let mut stack: Vec<char> = Vec::new();

    let mut cocok = true;
    for x in string.chars() {
        if x == '(' || x == '[' || x == '{' {
            stack.push(x);
        } else if (x == ')' || x == ']' || x == '}') && stack.len() > 0 {
            cocok = match stack.pop() {
                Some(last) => match map.get(&last) {
                    Some(&a) => is_same(x, a),
                    None => false
                }
                None => false
            };
        } else if (x == ')' || x == ']' || x == '}') && stack.len() == 0 {
            stack.push(x);
        }
    }

    cocok && stack.len() == 0
}

fn is_same(x: char, y: char) -> bool {
    x == y
}

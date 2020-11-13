use std::collections::HashMap;

fn main() {
    //                    p  s
    // [1, 2, 4, 4, 5, 6, 7, 3, 2, 7, 8, 9, 1]
    // [4, 5, 6, 7] -> len 4
    // [7, 8, 9] -> len = 3
    // jawaban = [4, 5, 6, 7]
    let s = [1, 2, 4, 4, 5, 6, 7, 3, 2, 7, 8, 9, 1];
    println!("{:#?}", longest_incrementing_subslice(&s));
}

pub fn longest_incrementing_subslice(s: &[u8]) -> &[u8] {
    let mut stack: Vec<u8> = Vec::new();
    let mut map = HashMap::new();
    let mut i = 0;
    let mut j = 1;
    let mut x = 0;

    let cnt = &s.len() - 2;
    loop {
        if s[i] < s[j] && s[j] - 1 == s[i] {
            stack.push(s[i]);
        } else if s[i] < s[j] && s[j] - 1 != s[i] {
            stack.push(s[i]);
            map.insert(x, stack.clone());
            stack.clear();
            x += 1;
        } else if s[i] == s[j] {
            stack.push(s[i]);
            map.insert(x, stack.clone());
            stack.clear();
            x += 1;
        } else if s[i] > s[j] {
            stack.push(s[i]);
            map.insert(x, stack.clone());
            stack.clear();
            x += 1;
        }

        i += 1;
        if j < cnt {
            j += 1;
        }

        if i == s.len() - 1 {
            break;
        }
    }

    let mut i = 0;
    let mut index = None;
    for (k, val) in map.iter() {
        if val.len() > i {
            i = val.len();
            index = Some(k);
        }
    }

    let x = map.get(index.unwrap()).unwrap().clone();
    &Box::new(x)
}

use std::collections::HashMap;

fn main() {
    let word = "regularly";
    let mut map: HashMap<char, i32> = HashMap::new();
    for i in word.chars() {
        if map.contains_key(&i) {
            if let Some(x) = map.get_mut(&i) {
                *x += 1;
            }
        } else {
            map.insert(i, 1);
        }
    }

    println!("{:#?}", map);
}

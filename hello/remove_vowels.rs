// use std::env;

fn main() {
    let word = String::from("Hello world");
    let res = remove_vowels(word);
    println!("{}", res);
}

fn remove_vowels(word: String) -> String {
    let mut res = String::new();
    for c in word.chars() {
        match c {
            'a' | 'i' | 'u' | 'e' | 'o' => {}
            _ => res.push(c),
        }
    }
    res
}

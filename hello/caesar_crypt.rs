use std::collections::HashMap;

fn main() {
}

fn create_shift_substitutions(n: i32) -> (HashMap<char, char>, HashMap<char, char>) {
    let mut encoding: HashMap<char, char> = HashMap::new();
    let mut decoding: HashMap<char, char> = HashMap::new();
    let ascii_char = get_ascii();
    let alphabet_size = &ascii_char.len();

    let nsize = n as usize;
    for i in 0usize..*alphabet_size {
        let letter = &ascii_char[i];
        let subst_letter = &ascii_char[(i + nsize) % alphabet_size];

        encoding[&letter] = *subst_letter;
        decoding[&subst_letter] = *letter;
    }

    (encoding, decoding)
}

fn get_ascii() -> Vec<char> {
    let mut res = Vec::new();
    let ascii = (b'A'..=b'Z').map(char::from);

    for x in ascii {
        res.push(x);
    }
    res
}
fn main() {
    let s: String = "Hello, world!".chars()
        .map(|x| match x {
            '!' => '?',
            'A'..='Z' => 'X',
            'a'..='z' => 'x',
            _ => x
        }).collect();
    println!("{}", s);
}
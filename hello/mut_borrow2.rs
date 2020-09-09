fn main() {
    let mut buffer: String = format!("Rustacean");
    for i in 0..buffer.len() {
        let slice = &buffer[i..];
        // buffer.push_str("s");
        println!("{:?}", slice);
    }
    buffer.push_str("s");
}

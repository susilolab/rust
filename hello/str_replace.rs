fn main() {
    let p = "../../.";
    println!("{}", p.replace("..", "").replace(".", ""));
}
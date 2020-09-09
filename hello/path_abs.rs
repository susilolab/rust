use std::path::Path;

fn main() {
    let s = Path::new("bin").is_absolute();
    println!("{:?}", s);

    let s = Path::new("/tmp").is_absolute();
    println!("{:?}", s);
}

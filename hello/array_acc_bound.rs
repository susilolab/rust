// mengakses array elemen dengan `match` agar dapat memilah apakah elemenny ada atau tidak
fn main() {
    let number = [1, 2, 3];
    match number.get(2) {
        Some(val) => println!("{}", val),
        None => println!("index 3 tidak ada!."),
    }
}

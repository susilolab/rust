// fungsi `filter` menghasilkan array baru
//
fn main() {
    let odd_number: Vec<i32> = (1..5).filter(|x| x % 2 == 0).collect();
    let sum: i32 = (1..5).filter(|x| x % 2 == 0).sum();
    println!("odd_number= {:?}", odd_number);
    // output: [2, 4]
    println!("sum= {}", sum);
    // output: [2, 6] kemudian di `sum` jadi 6
}

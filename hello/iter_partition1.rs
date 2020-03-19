// `partition` membuat data iterator menjadi dua
// 
fn main() {
    let a = [1, 2, 3, 4, 5];
    let (even, odd): (Vec<i32>, Vec<i32>) = a
        .iter()
        .partition(|&x| x % 2 == 0);

    println!("odd: {:?}", odd); 
    println!("even: {:?}", even); 
}
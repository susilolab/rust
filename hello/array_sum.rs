fn main() {
    let input: [i32; 6] = [1, 2, 3, 4, 10, 11];
    let mut sum: i32 = 0;
    for x in input.iter() {
        sum += x;
    }

    println!("sum: {}", sum);
}

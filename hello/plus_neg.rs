fn main() {
    // neg + pos
    let a = 160;
    let b = -20;
    println!("{:?}", a + b); // 140

    let c = -10;
    if c > 0 {
        println!("Positif");
    } else {
        println!("Negatif");
    }

    let d = -10 / 2;
    println!("{:?}", d);

    let a = -3000;
    let b = 30;
    println!("{:?}", b + a);

    // sisa bagi
    let a = -160;
    let b = a % -60;
    println!("{:?}", b);
}

use std::num::Wrapping;

fn main() {
    let i: i32 = -1;
    println!("{}", i);

    neg_num_compare();

    let x = Wrapping(3i32);
    let y = Wrapping(-3);
    println!("3 - (-3) = {:?}", x - y);
    println!("3 + -3 = {:?}", x + y);

    let a = Wrapping(3i32);
    let b = Wrapping(-30);
    println!("-30 + 3 = {:?}", b + a);

    // let x: i32 = -30; // kena shadowing jadi harus menyebutkan tipenya
    let y = -30;
    println!("abs(-30): {}", y.wrapping_abs());
}

fn neg_num_compare() {
    let i = Wrapping(-1);
    let j = Wrapping(-24);

    println!("{:?}", j < i);
}
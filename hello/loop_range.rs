fn main() {
    loop_from_to(6, 8);
}

fn loop_from_to(start: i32, end: i32) {
    let starts = start - 1;
    for x in (starts..=end).rev() {
        println!("{}", x);
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 10, y: 20 };
    
    // Memecah struct Point ke variabel a dan b
    let Point { x: a, y: b } = p;
    assert_eq!(10, a);
    assert_eq!(20, b);

    destruct_shorthand();
}

// Memecah struct dengan variabel terpisah secara singkat
//
fn destruct_shorthand() {
    let p = Point { x: 1, y: 2 };

    let Point { x, y } = p;
    println!("x: {}, y: {}", x, y);
}
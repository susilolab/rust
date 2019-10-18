fn main() {
    let mut v = Vec::new();
    v.push(1);
    push(&mut v);
    read(&v);
}

fn push(v: &mut Vec<i32>) {
    v.push(2);
    v.push(3);
}

fn read(v: &Vec<i32>) {
    for x in v {
        println!("{}", x);
    }
}
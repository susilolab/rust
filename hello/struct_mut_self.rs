#[derive(Debug)]
struct Seq {
    number: i32,
}

impl Seq {
    fn inc(&mut self) {
        self.number += 1;
    }
}

fn main() {
    let mut seq = Seq { number: 1 };
    println!("{:?}", seq);

    seq.inc();
    println!("{:?}", seq);
}
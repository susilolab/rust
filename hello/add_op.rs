// Penerapan operator `+` pada struk Count
// sehingga struk Count dapat dijumlah dengan struk Count yg lain
//
use std::ops::AddAssign;

#[derive(Debug)]
struct Count {
    value: i32,
}

impl AddAssign for Count {
    fn add_assign(&mut self, other: Count) {
        self.value += other.value;
    }
}

fn main() {
    let mut c1 = Count { value: 1 };
    let c2 = Count { value: 6 };

    c1 += c2;
    println!("{:?}", c1);
}

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}

fn main() {
    let mut seq = 0..3;

    println!("> {:?}", seq.next());
    println!("> {:?}", seq.next());
    println!("> {:?}", seq.next());
    println!("> {:?}", seq.next());

    println!("Iterate dari 0..3 menggunakan `for`");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("4 angka fibonaci pertama adalah");
    for i in fibonacci().take(4) {
        println!("> {}", i);
    }

    let arr = [1u32, 3, 3, 7];
    println!("Iterate array {:?}", &arr);
    for i in arr.iter() {
        println!("> {}", i);
    }
}


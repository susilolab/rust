struct S;

trait Hello {
    fn hello();
}

impl S {
    fn hello(self) {
        println!("Halo dari impl.");
    }
}

impl Hello for S {
    fn hello() {
        println!("Halo dari trait.");
    }
}

fn main() {
    let s = S {};
    s.hello();
    S::hello();
}

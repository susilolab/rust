struct S;

trait Hello {
    fn f(&self);
    fn g(&self);
}

impl S {
    fn f(&self) {
        println!("1");
    }

    fn g(&mut self) {
        println!("1");
    }
}

impl Hello for S {
    fn f(&self) {
        println!("2");
    }

    fn g(&self) {
        println!("2");
    }
}

fn main() {
    S.f();
    S.g();
}

struct Fib(u64, u64);

impl Fib {
    fn call_mut(&mut self) -> u64 {
        let res = self.0;
        self.0 = self.1;
        self.1 += res;
        res
    }

    fn call_once(mut self) -> u64 {
        self.call_mut()
    }
}

fn main() {
}

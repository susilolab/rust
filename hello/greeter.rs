struct Greeter;

impl Greeter {
    fn call(&self) {
        println!("Hello rust");
    }

    fn call_mut(&mut self) {
        self.call();
    }

    fn call_once(self) {
        self.call();
    }
}

fn main() {
    let mut greeter = Greeter {};
    greeter.call();
    // greeter.call_once();
    greeter.call_mut();
    greeter.call();
}

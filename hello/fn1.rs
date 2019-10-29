fn main() {
    let hello = || {
        println!("Hello fn1");
    };
    apply(hello);
}

fn apply<F>(f: F) where F: Fn() {
    f();
}

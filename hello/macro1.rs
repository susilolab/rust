macro_rules! say_hello {
    // `()` menandakan bahwa macro tidak punya arg
    () => {
        println!("Hello world!");
    };
}

fn main() {
    println!("Hello world!");
    say_hello!();
}

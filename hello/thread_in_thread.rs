use std::thread;

fn main() {
    // let child_one = thread::spawn(move || {
    //     println!("thread level 1.");

    //     let child_two = thread::spawn(move || {
    //         println!("thread level 2.");
    //     });
    //     child_two.join();
    // });
    // child_one.join();
    thread::Builder::new().name("child1".to_string()).spawn(move || {
        println!("Thread 1");
    });
}
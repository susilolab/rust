#[macro_use] extern crate tera;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate mysql;
extern crate serde_json;
extern crate redis;
extern crate actix;
extern crate actix_web;
extern crate actix_web_actors;
extern crate chrono;

use std::io;
use std::io::{Write};
use std::process;

mod basic;
mod redis_pubsub;
mod shadow;
mod cmd;
mod actixws;
mod fun;
mod dbmy;
mod hello_turtle;
mod datetime;

const MENUS: &'static str = "
~~ Rust playground ~~

1. Run cli command
2. Basic template menggunakan Tera
3. Demo redis pubsub
4. Contoh variabel shadow
5. Actix Websocket
6. Anonymouse func
7. Mysql demo
8. Turtle
9. Keluar
";

fn main() {
    show_menu();

    print!("Ketikan pilihan Anda [1-5]: ");
    io::stdout().flush();
    let mut user_choice: String = String::new();
    io::stdin().read_line(&mut user_choice)
        .expect("gagal membaca baris!.");
    let user_choice: i32 = user_choice.trim().parse()
        .expect("Mohon ketikan angka!.");
    println!("{}", user_choice);

    if user_choice == 1 {
        cmd::run_cmd();
        datetime::sekarang();

    } else if user_choice == 2 {
        basic::basic_templating();

    } else if user_choice == 3 {
        let ctx = redis_pubsub::Ctx::new();
        let handle = redis_pubsub::subscribe(&ctx);
        redis_pubsub::publish(&ctx);
        handle.join().unwrap();

    } else if user_choice == 4 {
        shadow::demo_shadow();
        shadow::trans_shadow();

    } else if user_choice == 5 {
        actixws::start_server();
    } else if user_choice == 6 {
        fun::fun_hello();
    } else if user_choice == 7 {
        dbmy::show_todo();
    } else if user_choice == 8 {
        hello_turtle::hello();
    } else if user_choice == 9 {
        process::exit(0);
    }
}

fn show_menu() {
    print!("{}[2J", 27 as char);
    println!("{}", MENUS);
}
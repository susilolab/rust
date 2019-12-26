use crate::model;
use std::process::Command;

pub fn run_task(channel: &str, msg: &str) {
    println!("Kanal '{}' menerima '{}'", channel, msg);
}

pub fn run_cli(model: model::Payload) {
    let output = Command::new(model.task_name)
        .args(model.args)
        .output()
        .expect("Tidak dapat menjalankan command.");
    let ls = output.stdout;
    println!("{:#?}", ls.iter().map(|&x| x as char).collect::<String>());
}

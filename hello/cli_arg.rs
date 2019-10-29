use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ls /home/susilo")
        .output()
        .expect("Tidak dapat menjalankan command!.");

    let ls = output.stdout;
    println!("{:#?}", ls.iter().map(|&x| x as char).collect::<String>());
}
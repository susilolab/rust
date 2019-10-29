use std::process::Command;

fn main() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("/home/susilo/var/www/ecc4/protected/yii-dev")
        .output()
        .expect("Tidak dapat menjalankan command!.");

    let ls = output.stdout;
    println!("{:#?}", ls.iter().map(|&x| x as char).collect::<String>());
}
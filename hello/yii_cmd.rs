use std::process::Command;

fn main() {
    run_with_cd();
}

fn run() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("/home/susilo/var/www/ecc4/protected/yii-dev")
        .output()
        .expect("Tidak dapat menjalankan command!.");

    let ls = output.stdout;
    println!("{:#?}", ls.iter().map(|&x| x as char).collect::<String>());
}

fn run_with_cd() {
    let output = Command::new("./yii-dev")
        .current_dir("/home/susilo/var/www/ecc4/protected")
        .output()
        .expect("Tidak dapat menjalankan command!.");

    let ls = output.stdout;
    println!("{:#?}", ls.iter().map(|&x| x as char).collect::<String>());
}

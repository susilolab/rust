use std::process::Command;

pub fn run_cmd() {
    let output = Command::new("bash")
        .arg("-c")
        .arg("ls /home/susilo")
        .output()
        .expect("gagal menjalankan proses");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}
/// du -h . --max-depth=0
///
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::env;

fn main() {
    let dirname = env::args()
        .nth(1)
        .unwrap_or_else(|| ".".to_string());

    duh(&dirname);
}

fn duh(path_name: &str) {
    let output = Command::new("du")
        .arg("-h")
        .arg("--max-depth=0")
        .arg(path_name)
        .output()
        .expect("Gagal menjalankan perintah");
    io::stdout().write_all(&output.stdout).unwrap();
}

#[allow(dead_code)]
fn get_files(dirname: &str) -> std::io::Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();

    for entry in fs::read_dir(dirname)? {
        let dir = entry?;
        res.push(dir.path().display().to_string());
    }
    Ok(res)
}

/// Compile Code
///
/// Ahad, 15 Desember 2019 18:15
/// Kompil kode rust
/// Todo:
/// 1. List semua file *.rs
/// 2. Jalankan command rustc nama_file.rs
/// 3. Catat error dan file yg gagal di kompile ke file
/// 4. Hapus file debug
///
use std::fs;
use std::process::Command;
use std::io::{Write};
use std::path::Path;
use std::fs::OpenOptions;
// use std::io::prelude::*;

#[derive(Debug)]
struct Output {
    err: Option<String>,
    out: Option<String>,
}

fn main() {
    let dirname = String::from(r#"/home/susilo/var/Rust/hello"#);
    if let Ok(files) = get_files(dirname) {
        for file in files {
            let fname = Path::new(&file);
            let ext = match fname.extension() {
                Some(val) => {
                    val.to_str().unwrap()
                },
                None => ""
            };

            if ext  == "rs" {
                let res = run_compile(file);
                println!("{:?}", res);
                if let Some(val) = res.err {
                    write_log("error_compile.log", val);
                }
            }
        }
    }
}

/// Get files
/// Mengembalikan daftar file dengan tipe Array String
/// Kembalian harus di unwrap atau match
fn get_files(dirname: String) -> std::io::Result<Vec<String>> {
    let mut res: Vec<String> = Vec::new();

    for entry in fs::read_dir(dirname)? {
        let dir = entry?;
        res.push(dir.path().display().to_string());
    }
    Ok(res)
}

fn run_compile(file_name: String) -> Output {
    let output = Command::new("rustc")
        .arg("--out-dir=/home/susilo/var/Rust/hello/bin")
        .arg(file_name)
        .output()
        .expect("Gagal menjalankan perintah rustc");
    let res_out = output.stdout.as_slice();
    let res_err = output.stderr.as_slice();
    let stdout = if String::from_utf8_lossy(&res_out).to_string().len() > 0 {
        Some(String::from_utf8_lossy(&res_out).to_string())
    } else {
        None
    };

    let stderr = if String::from_utf8_lossy(&res_err).to_string().len() > 0 {
        Some(String::from_utf8_lossy(&res_err).to_string())
    } else {
        None
    };

    Output {
        err: stderr,
        out: stdout,
    }
}


fn write_log(file_name: &str, content: String) {
    let file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(file_name);

    match file {
        Err(e) => eprintln!("{}", e),
        Ok(mut f) => {
            f.write_all(content.as_bytes());
        }
    }
}

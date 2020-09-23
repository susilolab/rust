use std::process::Command;

fn main() {
    let cmd = Command::new("unrar")
        .args(&["lb", "/home/susilo/Documents/eBooks/Rust/Packt.The.Complete.Rust.Programming.Reference.Guide.1838828109.rar"])
        .output();
        // .expect("Gagal menjalankan compiler");
        // unrar x -xNamaFile -c-

    match cmd {
        Ok(output) => {
            if output.status.success() {
                let s = output.stdout.iter().map(|&x| x as char).collect::<String>();
                let arr_s = s.split('\n').filter(|x| !x.is_empty()).collect::<Vec<&str>>();
                for x in arr_s {
                    println!("{}", x);
                }

            } else {
                println!("Unrar tidak berhasil.");
            }
        },
        Err(_) => println!("Gagal unrar file!.")
    }
}

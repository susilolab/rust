use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let _ = write_file();
    check_file_exist();
}

fn write_file() -> std::io::Result<()> {
    let mut file = File::create("/tmp/foo.txt")?;
    file.write_all(b"Hello world!")?;
    Ok(())
}

fn check_file_exist() {
    println!("{}", Path::new("/home/susilo/var/Elixir/upgradid/mix.exs").exists());
}
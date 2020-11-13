use std::{fs, io};

fn main() {
    let _ = list_folders("d:/var");
}

fn list_folders(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    println!("{:?}", entries);

    Ok(())
}
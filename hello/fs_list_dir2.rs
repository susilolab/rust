use std::fs;

#[derive(Debug)]
struct Files(String, String);

fn main() {
    // paths mengembalikan iterator dari std::fs:DirEntry
    let paths = fs::read_dir(r#"/run/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/susilo/DCIM/Camera"#).unwrap();
    let f = vec![
        String::from("/run/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/susilo/DCIM/Camera/IMG_20190727_173729.jpg"),
        String::from("/run/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/susilo/DCIM/Camera/IMG_20190821_222241.jpg")
        ];
    let p = paths
        .filter_map(Result::ok)
        .fold(Vec::new(), |mut acc, x| {
            acc.push(x.path().display().to_string());
            acc
        })
        .iter()
        .filter(|x| !f.contains(&x))
        .for_each(|x| {
            println!("{}", x)
        });
    //	for path in paths {
    //                path.unwrap() mengembalikan PathBuf
    //                                 V
    //		println!("Name: {}", path.unwrap().path().display());
    //                                          ^
    //                              display() dari impl PathBuf
    //	}
}

use std::path::Path;
use std::ffi::OsStr;

fn main() {
    assert_eq!(Some(OsStr::new("bin")), Path::new("/usr/bin").file_name());
    assert_eq!(Some(OsStr::new("hello.txt")), Path::new("/usr/hello.txt").file_name());

    let path = Path::new("hello.rs");
    if let Some(ext) = path.extension().unwrap().to_str() {
        println!("{}", ext);
    }
}

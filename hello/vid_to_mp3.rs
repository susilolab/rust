use std::path::Path;
use std::process::Command;

fn main() {
    let files = vec!["Message Passing and PubSub.mp4"];
    let src_path = "/home/susilo/Videos";
    for x in files {
        let path = Path::new(x);
        let path_name = format!("{}/{}", src_path.to_owned(), x);
        let ext = get_ext(&path);
        let out_filename = format!("{}/{}", &src_path.to_owned(), &x.replace(ext, "mp3"));

        println!("{}", x);
        let mut child = Command::new("ffmpeg")
            .arg("-i")
            .arg(path_name)
            .arg("-b:a")
            .arg("192K")
            .arg("-vn")
            .arg(out_filename)
            .spawn()
            .expect("failed to execute child");

        let _ = child.wait().expect("failed to wait on child");
    }
}

fn get_ext(file_name: &Path) -> &str {
    file_name.extension().unwrap().to_str().unwrap_or("")
}

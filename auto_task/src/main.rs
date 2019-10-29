use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::time::Duration;

struct App {
    src: &'static str,
    dst: FileType,
}

struct FileType {
    img: String,
    audio: String,
    video: String,
    doc: String,
    doc_book: String,
    archive: String,
}

impl FileType {
    fn new() -> Self {
        Self {
            img: "/home/susilo/Pictures".to_owned(),
            audio: "/home/susilo/Music".to_owned(),
            video: "/home/susilo/Videos".to_owned(),
            doc: "/home/susilo/Documents".to_owned(),
            doc_book: "/home/susilo/Documents/eBooks".to_owned(),
            archive: "/home/susilo/var/backup".to_owned(),
        }
    }
}

impl App {
    fn watch(&self) -> Result<()> {
        let (tx, rx) = channel();

        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(500)).unwrap();

        watcher.watch(self.src, RecursiveMode::Recursive).unwrap();

        loop {
            match rx.recv() {
                Ok(event) => {
                    println!("{:?}", event);
                    match event {
                        DebouncedEvent::Create(path) => {
                            handle_file(path, &self.dst)
                        },
                        DebouncedEvent::Write(path) => {
                            handle_file(path, &self.dst)
                        },
                        _ => (),
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    }
}

fn handle_file(path: PathBuf, ftype: &FileType) {
    let fpath = path.clone();
    let file_name = Path::new(path.to_str().unwrap())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let ext = Path::new(file_name).extension().unwrap().to_str().unwrap();
    let video = vec!["mp4", "mov", "mkv", "mpg", "mpeg"];
    let audio = vec!["mp3", "ogg", "wav", "flac"];
    let img = vec!["jpg", "jpeg", "png", "svg", "bmp"];
    let archive = vec![
        "rar", "zip", "tar", "tar.gz", "tar.xz", "tar.bz", "tar.bz2", "tgz",
    ];
    let doc = vec![
        "odt", "ods", "odp", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "pdf", "epub", "xlsm",
    ];

    let root = fpath.parent().unwrap().to_str().unwrap();
    let src = format!("{}/{}", root, file_name);
    if video.contains(&ext) {
        let dst = format!("{}/{}", &ftype.video, file_name);
        let _ = fs::copy(&src, &dst);
        let _ = fs::remove_file(&src);
        println!("Menyalin file {} ke {}", file_name, &ftype.video);
    } else if audio.contains(&ext) {
        let dst = format!("{}/{}", &ftype.audio, file_name);
        let _ = fs::copy(&src, &dst);
        let _ = fs::remove_file(&src);
        println!("Menyalin file {} ke {}", file_name, &ftype.audio);
    } else if img.contains(&ext) {
        let dst = format!("{}/{}", &ftype.img, file_name);
        let _ = fs::copy(&src, &dst);
        let _ = fs::remove_file(&src);
        println!("Menyalin file {} ke {}", file_name, &ftype.img);
    } else if archive.contains(&ext) {
        let dst = format!("{}/{}", &ftype.archive, file_name);
        let _ = fs::copy(&src, &dst);
        let _ = fs::remove_file(&src);
        println!("Menyalin file {} ke {}", file_name, &ftype.archive);
    } else if doc.contains(&ext) {
        let dst = format!("{}/{}", &ftype.doc, file_name);
        let _ = fs::copy(&src, &dst);
        let _ = fs::remove_file(&src);
        println!("Menyalin file {} ke {}", file_name, &ftype.doc);
    }
}

fn main() {
    let app = App {
        src: "/home/susilo/Downloads",
        dst: FileType::new(),
    };

    if let Err(e) = app.watch() {
        println!("error: {:?}", e)
    }
}

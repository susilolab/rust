use std::fs;

fn main() {
    let fname = "/media/susilo/381ed80f-3b52-4756-8881-9b77edc118ca/Backup Phone/20200316-1013-DCIM/DCIM/Camera/VID_20200225_071914.mp4";
    fs::remove_file(fname).unwrap_err();
}
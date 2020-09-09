use std::path::Path;

fn main() {
    assert_eq!(Path::new("/tmp/ok").exists(), false);
}

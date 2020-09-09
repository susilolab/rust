fn min(x: u8, y: u8) -> u8 {
    if x < y {
        x
    } else {
        y
    }
}

fn max(x: u8, y: u8) -> u8 {
    if x > y {
        x
    } else {
        y
    }
}

fn find_length(s: &[u8]) -> &[u8] {
    for (idx, val) in s.iter().enumerate() {
        println!("{} {}", idx, val);
    }

    let a: &[u8; 3] = &[1u8, 2u8, 3u8];
    a
}

fn main() {
    find_length(&[1u8, 2u8, 3u8]);
}

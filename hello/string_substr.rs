fn main() {
    let s = "kajian.mp4";
    // `find` mirip fungsi `strpos` pada PHP
    let len = s.find(".").unwrap();
    println!("{:?}", s.find(".").unwrap());

    let fname = &s[0..len];
    println!("{}", fname);
}
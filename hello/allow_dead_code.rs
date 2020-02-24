// membolehkan kode yg tak terpakai tanpa ada peringatan
// ada 2 cara:
// 1. #[allow(dead_code)]
//    menon-aktifkan kode tak terpakai untuk setiap fungsi. setiap fungsi 
//    harus ditambahkan jika tidak ingin ada peringatan.
// 2. #![allow(dead_code)]
//    menon-aktifkan kode tak terpakai secara menyeluruh, tidak harus per fungsi.
//    (perhatikan tanda !)
// untuk menon-aktifkan peringatan jika variable yg tak terpakai gunakan
// #![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let a = 10;
    let b = 20;
    println!("{}", b);
}

fn hello() {
    println!("Hello world!");
}
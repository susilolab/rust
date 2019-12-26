/// Borrow
///
/// Sabtu, 14 Desember 2019 21:11
/// Borrow meminjam variabel tanpa bisa merubah isi variabel (read only)
/// borrow var ditandai dengan &(ampersand). pada fungsi sebelum tipe data, pada variabel
/// sebelum nama var
/// dengan adanya borrow var akan dapat digunakan berkali-kali
///
fn main() {
    // buat var name
    let name = String::from("Agus");
    // transfer kepemilikan var name ke fungsi helper
    helper(name);
    // var name akan didrop disini dan tidak dapat digunakan lagi

    // Contoh borrowing
    // buat var name_ref
    let name_ref = String::from("Susilo");
    // pinjamkan ke fungsi helper_ref
    helper_ref(&name_ref);
    // pinjamkan ke fungsi helper_ref
    helper_ref(&name_ref);
    // var name masih bisa diakses disini
    println!("var masih bisa diakses: {}", name_ref);
    helper(name_ref);
    // var name didrop
    // helper_ref(&name_ref);
}

// arg name akan ditransfer kepemilikan karena tidak meminjam
fn helper(name: String) {
    println!("{}", name);
}

// arg name akan meminjam var dengan adanya tanda &
fn helper_ref(name: &String) {
    println!("{}", name);
}

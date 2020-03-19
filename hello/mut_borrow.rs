/// Belajar mutable borrow (reference)
///
/// Ahad, 15 Desember 2019 17:31
/// borrow  = meminjam
/// mutable = data dapat dirubah
/// mutable borrow berarti meminjam data yang dapat dirubah via reference var
/// tanda mutable borrow dengan "&mut type_data"
fn main() {
    // buat var name dapat dirubah (mutable)
    let mut name = String::from("Hello");
    // ubah name
    update(&mut name, String::from(" world!, "));
    // ubah name
    update(&mut name, String::from(" Agus"));
    update(&mut name, String::from(" Susilo."));
    // cetak name
    println!("{}", name);
}

/// arg name mutable ref/borrow dilihat dari tanda &mut
/// pada saat dipanggil var harus menyertakan &mut juga
fn update(name: &mut String, word: String) {
    name.push_str(&word);
}

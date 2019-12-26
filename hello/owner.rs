/// Ownership/ Kepemilikan
///
/// Ahad, 15 Desember 2019 17:37
/// Pada rust setiap nilai/value memiliki pemilik yaitu variabel
/// dan setiap variabel hanya bisa dipakai satu kali kemudian akan didrop
/// kecuali menggunakan borrow(reference) dan mutable borrow
///
fn main() {
    // buat var name dengan default immutable/ tidak dapat diubah
    let name = String::from("Hello");
    // transfer kepemilikan var name ke fungsi cetak
    cetak(name);
    // var name akan didrop setelah fungsi cetak
    // jika kita mencoba memanggil fungsi cetak lagi maka akan keluar error
    // value used after move
    // cetak(name); // coba uncomment baris ini
    // untuk dapat memakai var 2x baca kode owner1.rs
}

fn cetak(name: String) {
    println!("{}", name);
}

Mutable Borrow

name: String
Ownership: mengendalikan semua akses, akan dibebaskan setelah selesai

name: &String
Shared reference: Banyak pembaca, tanpa penulis

name: &mut String
Mutable reference: Tidak ada pembaca lain, satu penulis


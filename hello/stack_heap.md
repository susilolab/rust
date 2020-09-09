stack 
- data dan ukurannya diketahui, menyimpan data ke stack lebih cepat dari pada ke heap. karena tidak perlu mencari ruang yang pas dengan data dan ukurannya.
- stack datanya berurutan, LIFO. last in first out
- saat kode kita memanggil fungsi, nilai yang dilewatkan ke fungsi akan disimpan ke stack.

heap
- tidak berurutan/ acak.
- lebih lama jika menyimpan ke heap karena harus mencari ruang yang pas.

Ownership rules
- setiap nilai di rust punya variabel yaitu pemilik/owner
- hanya ada satu owner pada satu waktu
- saat pemilik keluar dari scope, maka nilai akan didrop.

Tipe data `String` disimpan dan di alokasi di heap

Tipe yang otomatis implement trait `Copy`
- Semua integer
- Boolean
- char
- Floating point
- Tuple, jika berisi tipe yang dapat dicopy

Perbedaan antar tipe data `str` dan `String` adalah `str` ukurannya diketahui dan karena diketahui maka akan disimpan distack. dan `str` bersifat immutable.
/// tipe i32 merupakan tipe data yang akan selalu disalin nilainya
///
/// oleh karena itu kode berikut jika dikompile akan tetep jalan
/// walaupun variabel `x` berkali-kali ditransfer ke fungsi `double` dan `add`
fn main() {
    let x = 5;
    let _y = double(x);
    println!("{}", x);

    let _z = add(x);
}

fn double(x: i32) -> i32 {
    x * 2
}

fn add(x: i32) -> i32 {
	x + 10
}
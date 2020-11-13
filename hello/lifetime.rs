// Lifetime parameter 'a di sebutkan pada parameter `x` dan `y`
// karena kedua variabel ini berhubungan dengan kembalian
// Pada Rust masa hidup variabel harus valid dan sama
//
// Sebagai contoh pada fungsi `longest` masa hidup parameter x
// dan y harus sama karena kedua parameter tsb berhubungan dengan
// kembalian
// Misal pada fungsi longest hanya mengembalikan dari parameter x
// maka parameter y tidak perlu disebutkan notasi liftimenya ('a)
//
fn main() {
    // variabel `string1` masa hidupnya di antara tanda { dan }
    // atau selama `string1` tidak berpindah kepemilikannya.
    let string1 = String::from("abcd");
    // begitu juga dengan variabel `string2`
    let string2 = "xyz";
    // Pada saat variabel `string1` dan `string2` digunakan oleh
    // fungsi longest sebagai reference maka masa hidup (lifetime)
    // dari parameter dan kembalian harus sama atau
    // kompiler Rust akan komplain.
    let result = longest(string1.as_str(), string2);

    println!("String terpanjang: {}", result);
}

// Jika parameter y tidak berhubungan dengan kembalian maka
// notasi lifetime tidak perlu disebutkan
// contoh:
fn longest_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

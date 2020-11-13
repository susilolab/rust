// mencari tahu tipe kembalian dari `str.parse`
//
fn main() {
    let i = "1";
    let i_num = i.parse::<i32>();
    println!("{:#?}", i_num);
    // Output:
    // Ok(1)
    // artinya tipe kembaliannya Result<F, <F as FromStr>::Err>
    // merujuk ke trait std::str::FromStr
    // fungsi `from_str` kembaliannya Result<i32, Err>
    // i32 karena pada saat parse kita menyebutkan tipe datanya
    // parse::<i32>()
}

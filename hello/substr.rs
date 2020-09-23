fn main() {
    let dtype = "bigint(20)";
    let dtype_arr = dtype.split(' ').collect::<Vec<&str>>();
    let ob = &dtype_arr[0].find('(').unwrap();

    println!("{:?}", ob);
    let s = &dtype_arr[0];
    let idx = *ob;
    let s = &s[0..idx];

    println!("{}", s);
}

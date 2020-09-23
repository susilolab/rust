fn main() {
    let s = "{[()]}";
    let s_arr: Vec<String> = s.chars().map(|x| x.to_string()).collect();
    println!("{:?}", s_arr);

    let s = "bigint(20)";
    let s_arr: Vec<&str> = s.split(' ').collect();
    println!("{:?}", s_arr);
}

fn main() {
    let a = ["1", "lol", "3", "NaN", "5"];
    let iter: Vec<_> = a
        .iter()
        // s.parse::<i32>() akan mengembalikan
        // Result<i32, Err>
        .map(|s| s.parse::<i32>())
        // akan memfilter dari `map` yg hasil hanya ok
        .filter_map(Result::ok)
        .collect();
    println!("{:?}", iter);
}

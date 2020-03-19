fn main() {
    let filter = |predicate: fn(&i32) -> bool, xs: Vec<i32>| {
        xs.into_iter().filter(predicate).collect::<Vec<i32>>()
    };

    let is_even = |x: &i32| x % 2 == 0;

    let res = filter(is_even, vec![1, 2, 3, 4, 5, 6]);
    println!("{:?}", res);
}

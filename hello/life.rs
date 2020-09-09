fn main() {}

fn get_int_ref() -> &'a i32 {
    let a: &'a i32 = 1;
    &a;
}

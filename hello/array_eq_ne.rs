fn main() {
    let array1 = [1, 2, 3];
    let array2 = [1, 2, 3];
    let array3 = [2, 3, 4];

    if array1.eq(&array2) {
        println!("array1 dan array2 sama!");
    }

    if array1.ne(&array3) {
        println!("array1 dan array3 tidak sama!");
    }
}

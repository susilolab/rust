fn main() {
    let s1 = "GAGCCTACTAACGGGAT";
    let s2 = "CATCGTAATGACGGCCT";

    // iterators are lazy and do nothing unless consumed
    let res = s1
        .chars()
        .zip(s2.chars()) //.for_each(|x| println!("{:?}", x));
        .fold(0, |acc, (x, y)| if x != y { acc + 1 } else { acc });

    println!("{:?}", res);
}

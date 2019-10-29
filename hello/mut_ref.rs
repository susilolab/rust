fn main() {
    let mut book = Vec::new();
    book.push(1);

    {
        let r = &mut book;
        // var book tidak dapat diakses krn masih di pinjam via muttable refernce
        // `&mut`
        // println!("{:?}", book.len());
        r.push(2);
        println!("{:?}", r);
    }
    book.push(3);
    println!("{:?}", book);
}
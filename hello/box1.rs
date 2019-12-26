/// std::boxed
/// tipe pointer untuk alokasi heap/tumpukan/stack
/// Box<T> biasa disebut `kotak`, menyediakan bentuk paling sederhana untuk
/// pengalokasian tumpukan di rust.
/// Boxes menyediakan kepemilikan untuk alokasi ini dan drop isinya saat mereka
/// keluar dari lingkup.
///
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let val: u8 = 5;
    let boxed: Box<u8> = Box::new(val);

    println!("{}", *boxed);

    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{:?}", list);
}

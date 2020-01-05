/// Contoh mengembalikan Self pada definisi trait sehingga
/// pada saat trait di implementasi bisa mengembalikan tipe sesuai
/// dengan struk yg meng-`implementasi` trait tsb
///
trait A {
    fn new(name: String) -> Self;
}

struct Person {
    name: String,
}

struct Animal {
    name: String,
}

impl A for Person {
    fn new(name: String) -> Person {
        Person { name: name }
    }
}

impl A for Animal {
    fn new(name: String) -> Animal {
        Animal { name: name }
    }
}

fn main() {
    let p: Person = Person::new("Agus".to_string());
    println!("{}", p.name);

    let a: Animal = Animal::new("Unta".to_string());
    println!("{}", a.name);
}

/// Contoh mengembalikan Self pada definisi trait sehingga
/// pada saat trait di implementasi bisa mengembalikan tipe sesuai
/// dengan struk yg meng-`implementasi` trait tsb
///
/// `Self` dengan huruf kapital "S" digunakan untuk merujuk pada tipe implementasi pada traits
/// Self hanya bisa digunakan sbg segmen pertama, tanpa awalan `::`
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

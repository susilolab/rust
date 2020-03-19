/// `dyn` membantu membedakan antara traits/object `trait` dengan `struct`
/// `&Foo, Box<Foo>` dan `impl Bar for Foo` itu ambigu, karena dari semuanya
/// bisa menjadi sebuah trait atau sebuah struct
///
/// dengan adanya `dyn` tidak lagi menjadi ambigu, karena `trait` dibedakan
/// oleh keyword `dyn`
///
/// trait objects (syntax baru dyn)
/// &Foo     => &dyn Foo
/// &mut Foo => &mut dyn Foo
/// Box<Foo> => Box<dyn Foo>
///
/// structs
/// &Bar
/// &mut Bar
/// Box<Bar>
///
struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "Mbeeek!"
    }
}

impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "Mowwwww!"
    }
}

// mengembalikan beberapa struk yg mengimplementasi `Animal`,
// tapi kita tidak tahu yang mana saat waktu dicompile
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("Kamu mengacak hewan dan dia berkata: {}", animal.noise());
}

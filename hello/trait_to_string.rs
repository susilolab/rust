// Implementasi trait `ToString` untuk struk `Person` untuk dapat menggunakan fungsi `to_string`
//
struct Person {
    name: String,
    age: u8,
}

impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("Nama saya {} dan umur saya {}", self.name, self.age)
    }
}

fn main() {
    let agus = Person { name: String::from("Agus"), age: 35 };
    println!("{}", agus.to_string());
}

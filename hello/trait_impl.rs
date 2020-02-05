// Mendemokan implementasi fungsi pada struk
// yang satu langsung ke struk, yang satunya pake `trait`
// rust akan memanggil default fungsi dari `impl` struk terlebih dahulu
// jadi fungsi pada `trait` akan di abaikan.
//
struct S;

trait Hello {
    fn hello(&self);
}

impl S {
    fn hello(&self) {
        println!("Halo dari impl.");
    }
}

impl Hello for S {
    fn hello(&self) {
        println!("Halo dari trait.");
    }
}

fn main() {
    // default rust akan memanggil fungsi dari `impl` bukan trait
    let s = S {};
    s.hello();

    // untuk memanggil fungsi dari trait maka diperlukan langkah berikut
    Hello::hello(&S); // atau
    <S as Hello>::hello(&S);
}

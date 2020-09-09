// Fully qualified syntax untuk mencegah ambigu/kebingungan
//
// Misal kita punya 2 trait `Pilot` dan `Wizard` yang sama-sama
// punya method `fly` dan kedua method diimplementasi ke struct yg sama yaitu `Human`
// sedangkan struk `Human` sendiri mempunyai method `fly`
// Bagaimana cara memanggil fungsi `fly` agar sesuai dengan yang kita mau?
//
trait Pilot {
	fn fly(&self);
}

trait Wizard {
	fn fly(&self);
}

struct Human;

impl Pilot for Human {
	fn fly(&self) {
		println!("Ini kaptenmu sedang berbicara!.");
	}
}

impl Wizard for Human {
	fn fly(&self) {
		println!("Naik!.");
	}
}

impl Human {
	fn fly(&self) {
		println!("*mengepakkan sayap*");
	}
}

fn main() {
	// method `fly` yang dipanggil adalah yang diimplementasi basic bukan trait
	let person = Human;
	person.fly();

	// Jika ingin memanggil fungsi `fly` sesuai trait maka perlu seperti dibawah ini
	Pilot::fly(&person);
	Wizard::fly(&person);
}

// Karena method `fly` menerima parameter `self`, jika tidak punya dua `types` yang meng-
// implementasi satu trait, Rust dapat mengetahui implementasi trait yang mana yang digunakan
// berdasarkan tipe dari `self`
// lanjutnya lihat `trait_fully_q2.rs`
// Fully qualified syntax untuk mencegah ambigu/kebingungan
// melanjutkan dari file `trait_fully_q.rs`
// bahwa pada trait Pilot terdapat parameter `self` sehingga rust tahu implementasi
// trait yang mana yang harus digunakan berdasarkan tipe `self`
//
// Nah bagaimana dengan trait `Animal` ini dimana fungsi baby_name tidak menerima
// parameter `self` ?
//
// Maka perlu digunakan `Fully Qualified Syntax`
// definisinya:
// <Type as Trait>::function(receiver_jika_method, arg_selanjutnya);
// 
trait Animal {
	fn baby_name() -> String;
}

struct Cow;

impl Cow {
	fn baby_name() -> String {
		String::from("Pedet")
	}
}

impl Animal for Cow {
	fn baby_name() -> String {
		String::from("Gudel")
	}
}

fn main() {
	println!("Anak sapi dipanggil {}", Cow::baby_name());
	println!("Anak sapi dipanggil {}", <Cow as Animal>::baby_name());
}

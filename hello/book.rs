// dari Book.rb
// 08/12/2019 04:54
//
struct Book {
	title: String,
	author: String,
}

impl Book {
	fn get_info(&self) {
		println!("Title: {}, Author: {}", self.title, self.author);
	}
}

fn main() {
	let book = Book{
		title: String::from("Halaqoh Cinta"),
		author: String::from("@teladanrasul")
	};
	book.get_info();
}

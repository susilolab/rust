/// Contoh borrowing struct
///
struct Book {
	title: String,
	chapters: Vec<String>
}

fn main() {
	let mut chapters = Vec::new();
	chapters.push("Chapter 1".to_owned());
	chapters.push("Chapter 2".to_owned());
	chapters.push("Chapter 3".to_owned());

	let book: Book = Book{title: "Rust".to_string(), chapters: chapters};
	print_book(&book);
}

fn print_book(book: &Book) {
	println!("Title: {}", book.title);
	for ch in &book.chapters {
		println!("{}", ch);
	}
}

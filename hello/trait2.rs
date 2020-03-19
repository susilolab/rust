// buat trait Summary dengan default implementasi dari method `summarize`
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct News;

// karena struk News tidak mendetilkan fungsi `summarize` maka akan
// menggunakan default implementasi dari trait `Summary`
impl Summary for News {}

fn main() {
    let news = News {};
    println!("Summary: {}", news.summarize());
}

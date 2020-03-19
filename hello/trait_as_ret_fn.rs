/// `impl NamaTrait` sebagai tipe kembalian fungsi
/// maka fungsi tsb dapat mengembalikan setipe yang mengimplementasi
/// `NamaTrait`
///
// buat trait Summary dengan default implementasi dari method `summarize`
trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct News;
struct FlashNews {
    title: String,
}

// karena struk News tidak mendetilkan fungsi `summarize` maka akan
// menggunakan default implementasi dari trait `Summary`
impl Summary for News {}

impl Summary for FlashNews {
    fn summarize(&self) -> String {
        format!("{}", self.title)
    }
}

// fungsi ini mengembalikan semua tipe data yg mengimplementasi trait
// `Summary`
fn return_summary(switch: bool) -> impl Summary {
    if switch {
        FlashNews {
            title: "Rust 1.40 sudah release!.".to_string(),
        }
    } else {
        FlashNews {
            title: "Belajar trait di rust".to_string(),
        }
    }
}

fn main() {
    let news = News {};
    println!("Summary: {}", news.summarize());

    // mengembalikan summarize dari struk FlashNews
    let sum = return_summary(false);
    println!("{}", sum.summarize());

    // mengembalikan summarize dari struk FlashNews
    let sum = return_summary(true);
    println!("{}", sum.summarize());
}

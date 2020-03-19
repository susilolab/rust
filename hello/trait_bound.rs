/// pada file trait_as_param.rs kita sudah menggunakan fungsi dengan parameternya trait
/// dengan menggunakan syntax `impl NamaTrait` pada parameternya
/// 
/// Nah trait_bound.rs ini akan mempersingkat syntax dari `trait_as_param.rs` menggunakan trait
/// bound syntax
/// trait bound syntax diletakkan sebagai generic function setelah nama fungsi
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
        format!("news title {}", self.title)
    }
}

// parameter `item` menerima semua tipe yang mengimplementasikan trait
// `Summary`
// NOTE: diganti dengan syntax `trait bound`
fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let news = News {};
    println!("Summary: {}", news.summarize());

    let flash_news = FlashNews {
        title: "Berita terkini".to_string(),
    };
    println!("Summary: {}", flash_news.summarize());

    notify(news);
    notify(flash_news);
}

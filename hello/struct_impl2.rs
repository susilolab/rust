struct App {
    api_key: String,
    api_secret: String,
}

impl App {
    fn new(api_key: String, api_secret: String) -> Self {
        App {
            api_key,
            api_secret,
        }
    }

    fn post(&self) {
        println!("{}", self.base_url());
    }

    fn base_url(&self) -> &'static str {
        "https://upgrad.id"
    }
}

fn main() {
    let app = App::new("a".to_string(), "b".to_string());
    app.post();
}

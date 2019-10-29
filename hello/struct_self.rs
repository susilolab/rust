struct App {
    src: &'static str,
}

impl App {
    fn watch(self) {
        println!("{}", self.src);
    }

    fn new() -> Self {
        Self {
            src: "/home/susilo",
        }
    }
}

fn main() {
    let app = App::new();
    app.watch();

    let app = App { src: "/" };
    app.watch();
}
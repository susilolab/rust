extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let winapp = Application::new(Some("com.github.gtk-rs.example.basic"), Default::default())
        .expect("faield");

    winapp.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("Gtk");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(|_| {
            println!("Clicked!");
        });

        window.add(&button);
        window.show_all();
    });

    winapp.run(&[]);
}

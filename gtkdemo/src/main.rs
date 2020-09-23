extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow,
    Button,
    WindowPosition,
    WindowType,
    Window,
    HeaderBar,
};
use std::process;

pub struct App {
    pub window: Window,
    pub header: Header,
}

pub struct Header {
    pub container: HeaderBar,
}

impl App {
    fn new() -> App {
        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();

        window.set_titlebar(Some(&header.container));
        window.set_title("App Name");
        window.set_wmclass("app-name", "App Name");
        Window::set_default_icon_name("iconname");

        window.connect_delete_event(move |_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        App { window, header}
    }
}

impl Header {
    fn new() -> Header {
        let container = HeaderBar::new();
        container.set_title(Some("App Name"));
        container.set_show_close_button(true);
        Header { container }
    }
}

fn main() {
    // let winapp = Application::new(
    //     Some("com.github.gtk-rs.example.basic"),
    //     Default::default()
    // ).expect("faield");

    // winapp.connect_activate(|app| {
    //     let window = ApplicationWindow::new(app);
    //     window.set_title("Gtk");
    //     window.set_position(WindowPosition::CenterAlways);
    //     window.set_default_size(350, 70);

    //     let button = Button::new_with_label("Click me!");
    //     button.connect_clicked(|_| {
    //         println!("Clicked!");
    //     });

    //     window.add(&button);
    //     window.show_all();
    // });

    // winapp.run(&[]);
    if gtk::init().is_err() {
        eprintln!("Gagal menginisiali Aplikasi GTK");
        process::exit(1);
    }

    let app = App::new();
    app.window.show_all();

    gtk::main();
}

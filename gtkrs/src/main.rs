//! # Basic Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and position, how to add a `button` to this `window` and how to connect signals with actions.

#![crate_type = "bin"]

extern crate gtk;

mod hello_gtk;
mod list_store;

fn main() {
    // hello_gtk::run();
    list_store::run();
}

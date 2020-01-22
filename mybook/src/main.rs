#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use rocket::Request;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;

#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[get("/")]
fn index() -> Template {
    let context = User { id: 1, name: String::from("Agus Susilo"), };
    Template::render("index", &context)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

#[get("/user")]
fn user() -> Json<User> {
    Json(User {
        id: 1,
        name: String::from("Agus susilo"),
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, user])
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}

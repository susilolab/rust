#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;
#[macro_use] extern crate chrono;
pub mod models;
pub mod schema;

use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use listenfd::ListenFd;
use dotenv::dotenv;
use diesel::prelude::*;
use std::env;
use serde::{Deserialize, Serialize};

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello Dunia!")
}

pub fn establish_conn() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diset");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error koneksi ke {}", database_url))
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/hello", web::get().to(hello))
            .route("/posts", web::get().to(show_posts))
            .route("/users", web::get().to(show_users))
            .route("/conversation", web::get().to(show_conversation))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:8088").unwrap()
    };
    
    server.run().unwrap();
}

fn show_posts() -> Result<HttpResponse> {
    use schema::posts::dsl::*;
    use models::post::*;

    let conn = establish_conn();
    let result = posts
        .filter(published.eq(1))
        .limit(5)
        .load::<Post>(&conn)
        .expect("Error");

    Ok(HttpResponse::Ok().json(result))
}

fn show_users() -> Result<HttpResponse> {
    use schema::ommu_users::dsl::*;
    use models::user::*;

    let conn = establish_conn();
    let result = ommu_users
        .limit(5)
        .load::<User>(&conn)
        .expect("Error");
    println!("{:?}", result);
    Ok(HttpResponse::Ok().json(result))
}

// TODO: cari cara parsing datetime yg isinya 0000-00-00 00:00:00
fn show_conversation() -> Result<HttpResponse> {
    use schema::ommu_conversations::dsl::*;
    use models::conversation::*;

    let conn = establish_conn();
    let result = ommu_conversations
        .limit(5)
        .load::<Conversation>(&conn)
        .expect("Error");
    println!("{:?}", result);
    Ok(HttpResponse::Ok().json(result))
}

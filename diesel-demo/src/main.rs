#[macro_use] extern crate diesel;
#[macro_use] extern crate serde;

use dotenv::dotenv;
use diesel::prelude::*;
use std::env;

pub mod schema;
pub mod model;

fn main() {
    println!("Hello, world!");

    let conn = establish_conn();
    show_conversation(&conn);
}

pub fn establish_conn() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL harus diset");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error koneksi ke {}", database_url))    
}

fn create_conversation(data: model::Conversation, conn: &MysqlConnection) {
}

fn show_conversation(conn: &MysqlConnection) {
    use schema::ommu_conversations::dsl::*;
    use model::*;

    let result = ommu_conversations
        // .filter(published.eq(1))
        .limit(20)
        .load::<Conversation>(conn)
        .expect("Error");

    for x in result {
        println!("{:?}", x);
    }

    let r: diesel::result::QueryResult<Conversation> = ommu_conversations.find(1).first(conn);
    println!("{:?}", r);
}
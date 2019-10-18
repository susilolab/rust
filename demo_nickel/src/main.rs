#[macro_use]
extern crate nickel;
extern crate hyper;
extern crate mysql;

use nickel::{Nickel, HttpRouter};
use hyper::method::Method;
use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct User {
	id: i32,
	username: String,
	password: String,
}

fn main() {
	let pool = my::Pool::new("mysql:://root:aq?pada*di^^^@127.0.0.1:3306")
	let mut server  = Nickel::new();

	server.utilize(explicit_router());
	server.add_route(Method::Get, "/bar", middleware! {
		"This is the /bar handler"
	});

	server.get("/:foo", middleware! { |request|
		let foo = request.param("foo").unwrap();
		let format = request.param("format").unwrap();
		format!("Foo is '{}'. the requested format is '{}'", foo, format);
	});
	server.listen("127.0.0.1:3030");
}

fn explicit_router() -> nickel::Router {
	let mut router = Nickel::router();

	router.get("/some/*/route", middleware! {
		"ini cocok dg /some/ok/route"
	});

	router.get("/some/**/route", middleware! {
		"ini cocok dg /some/ok/fine/route"
	});

	router
}
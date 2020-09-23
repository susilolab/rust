#[macro_use]
extern crate tera;

use actix_files as fs;
use actix_session::{CookieSession, Session};
use actix_utils::mpsc;
use actix_http::{body::Body, Response};
use actix_web::{
    dev::ServiceResponse,
    http::StatusCode,
    middleware::{
        self,
        errhandlers::{
            ErrorHandlerResponse,
            ErrorHandlers
        }
    },
    get,
    post,
    web,
    App,
    HttpServer,
    Responder,
    error,
    HttpRequest,
    HttpResponse,
    Error,
    Result
};
use std::{
    env,
    io,
    collections::HashMap
};
use tera::Tera;

mod models;
mod todo;

#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

#[post("/event")]
async fn capture_event(evt: web::Json<models::Event>) -> impl Responder {
    format!("dapat event {:?}", evt)
}

#[get("/temperature")]
async fn current_temperature() -> impl Responder {
    web::Json(models::Measurement { temperature: 42.3 })
}

#[post("/register")]
async fn register(form: web::Form<models::Register>) -> impl Responder {
    format!("Hello {} from {}!", form.username, form.country)
}

// http://localhost:8088/1/agus/index.html
#[get("/greeting/{id}/{name}/index.html")]
async fn greeting(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {:?}! id: {:?}", info.1, info.0)
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world!")
}

async fn hello(path: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &path)
}

async fn hello_tera(
    tmpl: web::Data<tera::Tera>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, Error> {
    let s = if let Some(name) = query.get("name") {
        let mut ctx = tera::Context::new();
        ctx.insert("name", &name.to_owned());
        ctx.insert("text", &"Welcome!".to_owned());
        tmpl.render("user.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template Error"))?
    } else {
        tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template Error"))?
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[get("/welcome")]
async fn welcome(session: Session, req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    let mut counter = 1;
    if let Some(count) = session.get::<i32>("counter")? {
        println!("Session value: {}", count);
        counter = count + 1;
    }

    session.set("counter", counter)?;
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/welcome.html"))
    )
}

async fn response_body(path: web::Path<String>) -> HttpResponse {
    let text = format!("Hello {}!", *path);

    let (tx, rx_body) = mpsc::channel();
    let _ = tx.send(Ok::<_, Error>(web::Bytes::from(text)));

    HttpResponse::Ok().streaming(rx_body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .wrap(middleware::Logger::default())
            .data(tera)
            .service(favicon)
            .service(welcome)
            .service(web::resource("/hello-tera").route(web::get().to(hello_tera)))
            .service(index)
            .service(greeting)
            .service(current_temperature)
            .service(capture_event)
            .service(register)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .service(
                web::resource("/async-body/{name}").route(web::get().to(response_body))
            )
            .route("/hello/{name}", web::get().to(hello))
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

fn error_handlers() -> ErrorHandlers<Body> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(
        res.into_response(response.into_body()),
    ))
}

fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> Response<Body> {
    let request = res.request();

    let fallback = |e: &str| {
        Response::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let tera = request.app_data::<web::Data<Tera>>().map(|t| t.get_ref());
    match tera {
        Some(tera) => {
            let mut context = tera::Context::new();
            context.insert("error", error);
            context.insert("status_code", res.status().as_str());
            let body = tera.render("error.html", &context);
            match body {
                Ok(body) => Response::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error)
    }
}
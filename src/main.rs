use actix_files::{Files, NamedFile};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use askama::Template;

#[actix_web::get("/login")]
async fn login() -> impl Responder {
    NamedFile::open_async("./static/login.html").await
}

#[actix_web::get("/register")]
async fn register() -> impl Responder {
    NamedFile::open_async("./static/register.html").await
}

#[actix_web::get("/home")]
async fn home() -> impl Responder {
    #[derive(Template)]
    #[template(path = "home.html")]
    struct HelloTemplate<'a> {
        name: &'a str,
    }

    let body = HelloTemplate { name: "World" };
    HttpResponse::Ok().body(body.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(login)
            .service(register)
            .service(home)
            .service(Files::new("/static", "./static").prefer_utf8(true))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

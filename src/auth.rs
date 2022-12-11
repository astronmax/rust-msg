use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};

use serde::Deserialize;

#[derive(Deserialize)]
struct LoginForm {
    login: String,
    password: String,
    remember: Option<String>,
}

#[actix_web::post("/check_login")]
async fn check_login(form: web::Form<LoginForm>) -> impl Responder {
    let login = &form.login;
    let password = &form.password;
    let remember = match &form.remember {
        Some(_s) => true,
        None => false,
    };

    println!("{} {} {}", login, password, remember);

    HttpResponse::Ok().body("Hello")
}

#[actix_web::get("/login")]
async fn login_get() -> impl Responder {
    NamedFile::open_async("./static/login.html").await
}

#[actix_web::get("/register")]
async fn register_get() -> impl Responder {
    NamedFile::open_async("./static/register.html").await
}

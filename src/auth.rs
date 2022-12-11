use actix_files::NamedFile;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use serde::Deserialize;

use mysql::prelude::*;
use mysql::*;

#[derive(Deserialize)]
struct LoginForm {
    login: String,
    password: String,
    remember: Option<String>,
}

fn valid_user(req: &HttpRequest, login: &String, password: &String) -> bool {
    let pool = req.app_data::<Pool>().unwrap();
    let mut conn = pool.get_conn().unwrap();

    let query = format!("SELECT password FROM `users` WHERE name = \"{}\"", login);
    let real_pass: String = match conn.query_first(query).unwrap() {
        Some(pw) => pw,
        None => return false,
    };

    if real_pass != *password {
        return false;
    }

    true
}

#[actix_web::post("/check_login")]
async fn check_login(req: HttpRequest, form: web::Form<LoginForm>) -> impl Responder {
    let login = &form.login;
    let password = &form.password;
    let remember = match &form.remember {
        Some(_s) => true,
        None => false,
    };

    if valid_user(&req, login, password) {
        todo!();
    }

    HttpResponse::MovedPermanently()
        .append_header(("Location", "http://127.0.0.1:8080/home"))
        .body("")
}

#[actix_web::get("/login")]
async fn login_get() -> impl Responder {
    NamedFile::open_async("./static/login.html").await
}

#[actix_web::get("/register")]
async fn register_get() -> impl Responder {
    NamedFile::open_async("./static/register.html").await
}

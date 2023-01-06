mod api;
mod auth;
mod server;
mod websock;

use actix::{Actor, Addr};
use actix_files::{Files, NamedFile};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use env_logger::Env;
use mysql::Pool;

#[actix_web::get("/ws/{username}")]
async fn websocket(
    username: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        websock::WsChatSession {
            addr: srv.get_ref().clone(),
            username: username.into_inner(),
        },
        &req,
        stream,
    )
}

#[actix_web::get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("./static/login.html").await
}

#[actix_web::get("/home/{username}/{chat}")]
async fn home(_: web::Path<(String, String)>) -> impl Responder {
    NamedFile::open_async("./static/home.html").await
}

#[actix_web::get("/login")]
async fn login() -> impl Responder {
    NamedFile::open_async("./static/login.html").await
}

#[actix_web::get("/register")]
async fn register() -> impl Responder {
    NamedFile::open_async("./static/register.html").await
}

#[actix_web::get("/search/{username}")]
async fn search(_: web::Path<String>) -> impl Responder {
    NamedFile::open_async("./static/search.html").await
}

fn get_mysql_uri() -> String {
    let mysql_root_pass = match std::env::var("MYSQL_ROOT_PASSWORD") {
        Ok(val) => val,
        Err(_e) => panic!("need env variable MYSQL_ROOT_PASSWORD"),
    };

    let mysql_host = match std::env::var("MYSQL_HOST") {
        Ok(val) => val,
        Err(_e) => panic!("need env variable MYSQL_HOST"),
    };

    let mysql_port = match std::env::var("MYSQL_PORT") {
        Ok(val) => val,
        Err(_e) => panic!("need env variable MYSQL_PORT"),
    };

    let mysql_db = match std::env::var("MYSQL_DB") {
        Ok(val) => val,
        Err(_e) => panic!("need env variable MYSQL_DB"),
    };

    return format!(
        "mysql://root:{}@{}:{}/{}",
        mysql_root_pass, mysql_host, mysql_port, mysql_db
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let url = get_mysql_uri();
    let pool = Pool::new(url.as_str()).unwrap();
    let srv = server::ChatServer::new(Box::new(pool.clone())).start();

    log::info!("Starting HTTP server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "./static").prefer_utf8(true))
            .app_data(web::Data::new(srv.clone()))
            .app_data(pool.clone())
            .service(index)
            .service(home)
            .service(login)
            .service(register)
            .service(search)
            .service(websocket)
            .service(api::valid_user)
            .service(api::get_messages)
            .service(api::get_chats)
            .service(api::get_users)
            .service(api::add_user)
            .service(api::check_token)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

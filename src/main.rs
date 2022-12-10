mod templates;

use std::vec;

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
    let chats = vec![
        templates::Chat {
            name: "astron".to_string(),
            status: true,
        },
        templates::Chat {
            name: "max".to_string(),
            status: false,
        },
        templates::Chat {
            name: "max2".to_string(),
            status: false,
        },
        templates::Chat {
            name: "max3".to_string(),
            status: false,
        },
    ];

    let messages = vec![
        templates::Message {
            author: "AAA".to_string(),
            text: "klsdkldklsds".to_string(),
        },
        templates::Message {
            author: "BBB".to_string(),
            text: "sowpwoqpwopwqwp[pop".to_string(),
        },
        templates::Message {
            author: "AAA".to_string(),
            text: "klsdkldklsds".to_string(),
        },
        templates::Message {
            author: "BBB".to_string(),
            text: "sowpwoqpwopwqwp[pop".to_string(),
        },
        templates::Message {
            author: "AAA".to_string(),
            text: "klsdkldklsds".to_string(),
        },
        templates::Message {
            author: "BBB".to_string(),
            text: "sowpwoqpwopwqwp[pop".to_string(),
        },
    ];

    let body = templates::HomeTemplate {
        messages: messages,
        chats: chats,
        owner: "Astronmax".to_string(),
    };

    HttpResponse::Ok().body(body.render().unwrap())
}

#[actix_web::get("/search")]
async fn search() -> impl Responder {
    let body = templates::SearchTemplate {};
    HttpResponse::Ok().body(body.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(login)
            .service(register)
            .service(home)
            .service(search)
            .service(Files::new("/static", "./static").prefer_utf8(true))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

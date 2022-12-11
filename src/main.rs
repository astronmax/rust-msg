mod auth;
mod templates;

use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use env_logger::Env;

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
    let users = vec![
        templates::User {
            name: "user_1".to_string(),
        },
        templates::User {
            name: "user_2".to_string(),
        },
        templates::User {
            name: "user_3".to_string(),
        },
        templates::User {
            name: "user_4".to_string(),
        },
        templates::User {
            name: "user_5".to_string(),
        },
    ];

    let body = templates::SearchTemplate { users: users };
    HttpResponse::Ok().body(body.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(auth::login_get)
            .service(auth::check_login)
            .service(auth::register_get)
            .service(home)
            .service(search)
            .service(Files::new("/static", "./static").prefer_utf8(true))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

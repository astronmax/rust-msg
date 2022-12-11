mod auth;
mod entity;
mod templates;

use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use env_logger::Env;
use mysql::Pool;

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

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
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

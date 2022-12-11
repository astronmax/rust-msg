use askama::Template;

pub struct Chat {
    pub name: String,
    pub status: bool,
}

pub struct Message {
    pub author: String,
    pub text: String,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub chats: Vec<Chat>,
    pub messages: Vec<Message>,
    pub owner: String,
}

pub struct User {
    pub name: String,
}

#[derive(Template)]
#[template(path = "search.html")]
pub struct SearchTemplate {
    pub users: Vec<User>,
}

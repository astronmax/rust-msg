use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Chat {
    pub user_id: usize,
    pub peer_id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Messages {
    pub chat_id: usize,
    pub author_id: usize,
	pub text: String,
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub roles: String,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token(pub String);
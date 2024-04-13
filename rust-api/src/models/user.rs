use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_salt: String,
    pub password_hash: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAuth {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DGS{
    pub id: i32,
    pub label: String
}


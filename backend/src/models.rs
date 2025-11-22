use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Account {
    pub id: Option<i64>,
    pub user_id: i64,
    pub app: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct AccountPayload {
    pub app: String,
    pub url: String,
    pub username: String,
    pub password: String,
}

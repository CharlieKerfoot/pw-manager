use axum::{
    extract::{State, Json},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
};
use sqlx::SqlitePool;
use crate::models::{Account, User, Claims};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use std::time::{SystemTime, UNIX_EPOCH};

const SECRET_KEY: &[u8] = b"super_secret_key_change_me"; // TODO: Move to env var

// --- Auth Handlers ---

pub async fn register(
    State(pool): State<SqlitePool>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password_hash.as_bytes(), &salt).unwrap().to_string();

    let result = sqlx::query("INSERT INTO users (email, password_hash) VALUES (?, ?)")
        .bind(&payload.email)
        .bind(password_hash)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<User>,
) -> impl IntoResponse {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
        .unwrap();

    if let Some(user) = user {
        let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
        if Argon2::default().verify_password(payload.password_hash.as_bytes(), &parsed_hash).is_ok() {
            let expiration = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize + 3600 * 24; // 24 hours

            let claims = Claims {
                sub: user.id.unwrap().to_string(),
                exp: expiration,
            };

            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).unwrap();
            return (StatusCode::OK, Json(serde_json::json!({ "token": token })));
        }
    }

    (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Invalid credentials" })))
}

// --- Helper to extract user_id from token ---
fn get_user_id(headers: HeaderMap) -> Option<i64> {
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                let validation = Validation::default();
                if let Ok(token_data) = decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation) {
                    return token_data.claims.sub.parse::<i64>().ok();
                }
            }
        }
    }
    None
}

// --- Account Handlers ---

pub async fn get_accounts(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user_id = match get_user_id(headers) {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Account>>(vec![])).into_response(),
    };

    let accounts = sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(&pool)
        .await
        .unwrap_or(vec![]);

    (StatusCode::OK, Json(accounts)).into_response()
}

pub async fn create_account(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    Json(payload): Json<crate::models::AccountPayload>,
) -> impl IntoResponse {
    let user_id = match get_user_id(headers) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED,
    };

    let result = sqlx::query(
        "INSERT INTO accounts (user_id, app, url, username, password) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(&payload.app)
    .bind(&payload.url)
    .bind(&payload.username)
    .bind(&payload.password)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_account(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    axum::extract::Path(id): axum::extract::Path<i64>,
) -> impl IntoResponse {
    let user_id = match get_user_id(headers) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED,
    };

    let result = sqlx::query("DELETE FROM accounts WHERE id = ? AND user_id = ?")
        .bind(id)
        .bind(user_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn update_account(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
    axum::extract::Path(id): axum::extract::Path<i64>,
    Json(payload): Json<crate::models::AccountPayload>,
) -> impl IntoResponse {
    let user_id = match get_user_id(headers) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED,
    };

    let result = sqlx::query(
        "UPDATE accounts SET app = ?, url = ?, username = ?, password = ? WHERE id = ? AND user_id = ?",
    )
    .bind(&payload.app)
    .bind(&payload.url)
    .bind(&payload.username)
    .bind(&payload.password)
    .bind(id)
    .bind(user_id)
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_all_accounts(
    State(pool): State<SqlitePool>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user_id = match get_user_id(headers) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED,
    };

    let result = sqlx::query("DELETE FROM accounts WHERE user_id = ?")
        .bind(user_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

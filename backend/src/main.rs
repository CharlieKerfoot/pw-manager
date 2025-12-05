mod handlers;
mod models;

use axum::{
    routing::get,
    Router,
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::net::SocketAddr;
use std::str::FromStr;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:database.db".to_string());

    let connection_options = SqliteConnectOptions::from_str(&database_url)
        .expect("Failed to parse database URL")
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connection_options)
        .await
        .expect("Failed to connect to database");

    // Create table if not exists
    // Create tables if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            app TEXT NOT NULL,
            url TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create tables");

    let app = Router::new()
        .route("/register", axum::routing::post(handlers::register))
        .route("/login", axum::routing::post(handlers::login))
        .route("/accounts", get(handlers::get_accounts).post(handlers::create_account))
        .route("/accounts/all", axum::routing::delete(handlers::delete_all_accounts))
        .route("/accounts/:id", axum::routing::delete(handlers::delete_account).put(handlers::update_account))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse::<u16>().expect("PORT must be a number");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

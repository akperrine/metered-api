mod app;
mod db;
mod error;
mod middleware;
mod models;
mod routes;

#[cfg(test)]
mod tests;
use middleware::auth::{create_auth_token, validate_auth_token};

use crate::db::connection;

#[tokio::main]
async fn main() {
    let token = create_auth_token();
    // validate_auth_token(token);

    // switching to user-base making sure works
    connection().await;

    let app = app::create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("local host runing on port 3000");

    axum::serve(listener, app).await.unwrap()
}

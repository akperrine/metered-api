mod app;
mod db;
mod models;

use crate::db::connection;

#[tokio::main]
async fn main() {
    connection().await;

    let app = app::create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}

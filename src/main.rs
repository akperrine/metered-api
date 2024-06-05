mod app;
mod db;
mod error;
mod middleware;
mod models;
mod routes;

#[cfg(test)]
mod tests;

use crate::db::connection;

#[tokio::main]
async fn main() {
    connection().await;

    let app = app::create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("local host runing on port 3000");

    axum::serve(listener, app).await.unwrap()
}

mod app;
mod db;
mod error;
mod models;
mod routes;

#[cfg(test)]
mod tests;

use std::str::FromStr;

use futures_util::AsyncReadExt;
use mongodb::bson::{oid::ObjectId, Bson};

use crate::db::{connection, get_bucket};

#[tokio::main]
async fn main() {
    connection().await;

    let app = app::create_app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("local host runing on port 3000");

    axum::serve(listener, app).await.unwrap()
}

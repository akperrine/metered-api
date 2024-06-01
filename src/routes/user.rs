use anyhow::Ok;
use axum::{
    body::Body,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use bson::{doc, Document};
use mongodb::Collection;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{db::connection, models::user::User};

pub fn create_route() -> Router {
    Router::new()
        .route("/users/:id", get(dummy_fn))
        .route("/users/create", post(create_user))
}

async fn dummy_fn() {
    println!("hello from users");
}

pub async fn create_user(Json(body): Json<CreateUserBody>) -> impl IntoResponse {
    let db = connection().await;
    // let client = Client::with_uri_str("uri")
    let user = User::new(body.username, body.email, body.password);

    let collection: Collection<User> = db.collection("users");
    // validate
    let found = collection
        .find_one(doc! {"email": &user.email}, None)
        .await
        .unwrap();
    println!("found: {:?}", found);

    if let Some(_) = found {
        // return Json("Username: {} already exists");
        return (StatusCode::BAD_REQUEST, Json("username already exists"));
    }
    let inserted = collection.insert_one(&user, None).await.unwrap();
    println!("inserted: {:?}", inserted);

    (StatusCode::BAD_REQUEST, Json("user successfully loaded"))
}

#[derive(Debug, Deserialize)]
struct CreateUserBody {
    username: String,
    email: String,
    password: String,
}

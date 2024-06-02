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

use crate::{
    db::connection,
    models::user::{user_password_hash, PublicUser, User},
};

pub fn create_route() -> Router {
    Router::new()
        .route("/users/:id", get(dummy_fn))
        .route("/users/login", get(get_user_by_email))
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
        return (StatusCode::BAD_REQUEST, Json("username already exists"));
    }
    let inserted = collection.insert_one(&user, None).await.unwrap();
    println!("inserted: {:?}", inserted);

    (StatusCode::BAD_REQUEST, Json("user successfully loaded"))
}

async fn get_user_by_email(Json(body): Json<LoginUserBody>) -> impl IntoResponse {
    if body.email == "" || body.password == "" {
        return (
            StatusCode::BAD_REQUEST,
            "incorrect credentails passed A.".to_string(),
        );
    }
    let db = connection().await;
    let collection: Collection<User> = db.collection("users");
    let found = collection
        .find_one(doc! {"email": body.email}, None)
        .await
        .unwrap();

    if let Some(user) = found {
        if bcrypt::verify(&body.password, &user.password).unwrap() {
            let response_body = PublicUser {
                id: user.id,
                username: user.username,
                email: user.email,
            };
            return (
                StatusCode::OK,
                serde_json::to_string(&response_body).unwrap(),
            );
            // );
        } else {
            return (
                StatusCode::BAD_REQUEST,
                "incorrect credentails passed Bs.".to_string(),
            );
        }
    }

    // (
    //     StatusCode::BAD_REQUEST,
    //     Json("incorrect credentials passed C."),
    // )
    (
        StatusCode::BAD_REQUEST,
        "incorrect credentails passed Cs.".to_string(),
    )
}

#[derive(Debug, Deserialize)]
pub struct CreateUserBody {
    username: String,
    email: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserBody {
    email: String,
    password: String,
}

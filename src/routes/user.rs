use axum::{
    extract::Multipart,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum_macros::debug_handler;
use bson::doc;
use mongodb::Collection;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    db::connection,
    middleware::auth::{create_auth_token, AuthError},
    models::user::{PublicUser, User},
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
        return (StatusCode::BAD_REQUEST, Json("invalid user or password"));
    }
    let inserted = collection.insert_one(&user, None).await.unwrap();
    println!("inserted: {:?}", inserted);

    (StatusCode::CREATED, Json("user successfully loaded"))
}

async fn get_user_by_email(
    Json(body): Json<LoginUserBody>,
) -> Result<Json<AuthResponse>, AuthError> {
    if body.email == "" || body.password == "" {
        return Err(AuthError::MissingCredintials);
    }
    let db = connection().await;
    let collection: Collection<User> = db.collection("users");
    let found = collection
        .find_one(doc! {"email": body.email}, None)
        .await
        .unwrap();

    if let Some(user) = found {
        if bcrypt::verify(&body.password, &user.password).unwrap() {
            let token = create_auth_token().await;
            return Ok(Json(AuthResponse::new(&token, user)));
        }
    }
    Err(AuthError::WrongCredentials)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserBody {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserBody {
    pub(crate) email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub user: PublicUser,
}

impl AuthResponse {
    fn new(token: &str, user: User) -> Self {
        Self {
            access_token: String::from(token),
            user: PublicUser::from(user),
        }
    }
}

// impl<AuthResponse> IntoResponse for AuthResponse {
//     fn into_response(self) -> Response<AuthResponse> {
//         Response::builder()
//             .status(StatusCode::OK)
//             .header("Content-Type", "application/json")
//             .body(self)
//             .unwrap()
//     }
// }

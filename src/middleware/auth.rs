// Require Claims

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{response::Response, Json};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

const SECRET: &str = "secret";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    exp: u64,
    iat: u64,
}

impl Claims {
    pub fn new() -> Self {
        Self {
            iat: jsonwebtoken::get_current_timestamp(),
            exp: jsonwebtoken::get_current_timestamp() + 86400,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthBody {
    header: Header,
    payload: Claims,
    // don't feel good about sig type
    signature: String,
}

pub async fn create_auth_token() -> String {
    let claims = Claims::new();

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )
    .unwrap();

    println!("token: {:?}", token);
    token
}

pub fn validate_auth_token(token: String) {
    let res = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret("SECRET.as_ref()".as_bytes()),
        &Validation::default(),
    )
    .unwrap();

    println!("{:?}", res);
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredintials,
    InvalidToken,
    InvalidTokenCreation,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong Credntials"),
            AuthError::MissingCredintials => (StatusCode::BAD_REQUEST, "Missing Credentials"),
            AuthError::InvalidTokenCreation => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Error Creating Token")
            }
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid Token"),
        };
        let body = Json(json!({
            "error": message
        }));
        (status, body).into_response()
    }
}

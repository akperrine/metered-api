use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::bson::doc;

use crate::middleware::auth::{validate_auth_token, AuthError};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PublicUser {
    pub id: ObjectId,
    pub username: String,
    #[validate(email)]
    pub email: String,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap(),
            username: user.username.clone(),
            email: user.email.clone(),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for PublicUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let token_data =
            validate_auth_token(bearer.token()).map_err(|_| AuthError::InvalidToken)?;

        let user = PublicUser {
            id: ObjectId::new(),
            email: String::from("e@.com"),
            username: String::from("hi"),
        };
        println!("{:?}", token_data.claims);
        Ok(user)
    }
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        let hashed_password = user_password_hash(&password);

        Self {
            id: None,
            username,
            email,
            password: hashed_password,
        }
    }
}

pub fn user_password_hash(unhashed: &String) -> String {
    let cost = 10;
    bcrypt::hash(unhashed, cost).unwrap()
}

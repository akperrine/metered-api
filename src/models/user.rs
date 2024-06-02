use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::bson::doc;

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
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[validate(email)]
    pub email: String,
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

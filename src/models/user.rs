use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::bson::doc;
use wither::{bson::oid::ObjectId, Model};

#[derive(Debug, Model, Serialize, Deserialize, Validate)]
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
        let cost = 10;
        let hashed_password = bcrypt::hash(password, cost).unwrap();

        Self {
            id: None,
            username,
            email,
            password: hashed_password,
        }
    }
}

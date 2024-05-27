use serde::{Deserialize, Serialize};
use validator::Validate;
use wither::bson::doc;
use wither::{bson::oid::ObjectId, Model};

#[derive(Debug, Model, Serialize, Deserialize, Validate)]
#[model(index(keys=r#"doc!{"email": 1}"#, options=r#"doc!{"unique": true}"#r))]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    #[validate(email)]
    pub email: String,
    pub password: Option<String>,
}

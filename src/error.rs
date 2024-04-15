use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tokio::task::JoinError;
use wither::bson;
use wither::mongodb::error::Error as MongoError;
use wither::WitherError;

// #[derive(thiserror::Error)]
pub enum Error {
    // #[error("request path not found")]
    NotFound,
    // #[error("internal server error occured")]
    Anyhow(anyhow::Error),
}

// impl IntoResponse for Error {
//     fn into_response(self) -> Response {}
// }

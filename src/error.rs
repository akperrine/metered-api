// use axum::http::StatusCode;
// use axum::response::{IntoResponse, Response};

// #[derive(thiserror::Error, Debug)]
// #[error("...")]
// pub enum AppError {
//     #[error("{0}")]
//     NotFound(#[from] NotFound),
//     #[error("{0}")]
//     BadRequest(#[from] BadRequest),
//     // #[error("internal server error occured")]
//     // Anyhow(anyhow::Error),
// }

// impl IntoResponse for AppError {
//     fn into_response(self) -> Response {
//         match self {
//             Self::NotFound(err) => (
//                 StatusCode::NOT_FOUND,
//                 format!("Resource not found: {}", err),
//             ),
//             Self::BadRequest(err) => (
//                 StatusCode::BAD_REQUEST,
//                 format!("Bad request made: {}", err),
//             ),
//             // Self::Anyhow(err) => (
//             //     StatusCode::INTERNAL_SERVER_ERROR,
//             //     format!("Something went wrong: {}", err),
//             // ),
//         }
//         .into_response()
//     }
// }

// #[derive(thiserror::Error, Debug)]
// #[error("Bad Request")]
// pub struct BadRequest;

// #[derive(thiserror::Error, Debug)]
// #[error("Not found")]
// pub struct NotFound;

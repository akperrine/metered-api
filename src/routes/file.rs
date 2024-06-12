use axum::{
    routing::{delete, get, post},
    Router,
};

pub fn create_route() -> Router {
    Router::new()
        .route("/files/:id", get(get_file_by_id))
        .route("/files/view/:file_id", get(get_file_as_view))
        .route("/files/add", post(post_new_file))
        .route("/files/remove", delete(delete_file))
}

pub async fn get_file_by_id() {}

pub async fn get_file_as_view() {}

pub async fn post_new_file() {}

pub async fn delete_file() {}

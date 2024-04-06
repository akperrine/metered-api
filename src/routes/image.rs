use axum::{
    routing::{get, post},
    Router,
};

pub fn create_routes() -> Router {
    Router::new()
        .route("/images/:id", get(dummy_fn))
        .route("/images", post(dummy_fn))
}

pub async fn dummy_fn() {
    println!("hello world");
}

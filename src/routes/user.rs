use axum::{http::Response, routing::get, Router};

pub fn create_route() -> Router {
    Router::new().route("/users/:id", get(dummy_fn))
}

async fn dummy_fn() {
    println!("hello from users");
}

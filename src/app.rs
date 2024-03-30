use axum::{routing::get, Router};

pub async fn create_app() -> Router {
    Router::new().route("/health_check", get(root))
}

async fn root() -> &'static str {
    "Healthy!"
}

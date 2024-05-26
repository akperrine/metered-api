use std::env;

use crate::{
    app::{self, create_app},
    db::{connection, create_bucket, get_bucket, get_env_config},
};
use lazy_static::lazy_static;
use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection, Database,
};
use once_cell::sync::Lazy;
use tokio::{runtime::Runtime, sync::OnceCell};

static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());
static API: OnceCell<()> = OnceCell::const_new();

pub async fn start_test_api() {
    API.get_or_init(|| async {
        env::set_var("RUN_MODE", "test");

        let db = connection().await;
        let app = app::create_app().await;
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
        println!("local host runing on port 3001");

        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
    })
    .await;
}

pub fn use_test_app<F>(test: F)
where
    F: std::future::Future,
{
    RUNTIME.block_on(async move {
        start_test_api().await;
        let db = connection().await;
        let image_bucket = get_bucket().await.unwrap();
        image_bucket.drop().await.unwrap();
        let user_collection: Collection<Document> = db.collection("users");
        let _ = user_collection.delete_many(doc! {}, None).await;
        println!("collections {:?}", db.list_collection_names(None).await);

        create_bucket(&db).await;
        test.await;
    })
}

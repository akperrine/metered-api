use std::env;

use crate::{
    app::{self, create_app},
    db::{connection, create_bucket, get_bucket, get_env_config},
};
use lazy_static::lazy_static;
use mongodb::{
    bson::Document,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection, Database,
};
use tokio::{runtime::Runtime, sync::OnceCell};

// static MOCK_CONNECTION: OnceCell<()> = OnceCell::const_new();

pub async fn start_test_api() {
    // MOCK_CONNECTION
    // .get_or_init(|| async {
    env::set_var("RUN_MODE", "test");

    let db = connection().await;
    let app = app::create_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("local host runing on port 3001");

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    // })
    // .await;

    // axum::Server::bind(&address)
    //     .serve(app.into_make_service())
    //     .await
    //     .expect("Failed to start server");
}

lazy_static! {
    static ref RUNTIME: Runtime = Runtime::new().unwrap();
}

pub async fn use_test_app<F>(test: F)
where
    F: std::future::Future,
{
    // RUNTIME.block_on(async move {
    println!("hiiiiii");
    start_test_api().await;
    let db = connection().await;
    let image_bucket = get_bucket().await.unwrap();
    image_bucket.drop().await;
    create_bucket(&db);
    // println!("db {:?}", db);
    println!("collections {:?}", db.list_collection_names(None).await);
    // let a = db.drop(None).await.unwrap();
    // println!("dropped `{:?}`", a);
    test.await;
    // })

    //
}

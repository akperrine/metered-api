use std::env;

use crate::{
    app::{self, create_app},
    db::{connection, get_env_config},
};
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use tokio::sync::OnceCell;

// static MOCK_CONNECTION: OnceCell<()> = OnceCell::const_new();

pub async fn start_test_api() {
    env::set_var("RUN_MODE", "test");

    connection().await;
    let app = app::create_app().await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("local host runing on port 3001");

    axum::serve(listener, app).await.unwrap()

    // let config = get_env_config();
    // let mut client_options = ClientOptions::parse_async(config.mongo_url).await.unwrap();

    // let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    // client_options.server_api = Some(server_api);

    // MOCK_CONNECTION
    //     .get_or_init(|| async {
    //         let client = Client::with_options(client_options).unwrap();

    //         let app = create_app().await;
    //         let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
    //             .await
    //             .map_err(|e| format!("Error binding TCP listener: {:?}", e))
    //             .unwrap();

    //         tokio::spawn(async move { axum::serve(listener, app) });
    //     })
    //     .await;
}

use std::env::{self};

use axum::extract::Multipart;
use image::buffer;
use mongodb::options::ClientOptions;
use reqwest::{Body, Client, StatusCode};
use tokio::{fs::File, io::AsyncReadExt};
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::{db::get_env_config, tests::setup::use_test_app};

// #[test]
// fn answer_42() {
//     assert_eq!(42, 42);
// }
// #[tokio::test]
// async fn test_connection() {
//     let config = get_env_config();

//     let mut client_options = ClientOptions::parse_async(config.mongo_url).await.unwrap();
//     let client = mongodb::Client::with_options(client_options);
//     print!("HI");

//     assert!(client.is_ok());
// }

#[tokio::test]
async fn test_post_get_delete_png() {
    use_test_app(async move {
        println!("{}", env::current_dir().unwrap().display());
        let test_img = File::open("./src/tests/images/test_img_1.png")
            .await
            .map_err(|_| "Cannot find test image")
            .unwrap();

        // read file body stream
        let stream = FramedRead::new(test_img, BytesCodec::new());
        let file_body = Body::wrap_stream(stream);
        //make form part of a file
        let some_file = reqwest::multipart::Part::stream(file_body)
            .file_name("test_img_1.png")
            .mime_str("image/png")
            .unwrap();
        // create multipart form
        let form = reqwest::multipart::Form::new().part("file", some_file);

        let client = Client::new();

        let res = client
            .post("http://localhost:3001/images")
            .multipart(form)
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let expected = StatusCode::OK;
        println!("{},{}", status_code, expected);
        assert_eq!(status_code, expected);
    })
    .await;
}
